use crate::can::protocol::message::{Message, SMMIdent, SMMPayload, SystemManagementMessage};
use crate::can::CANInterface;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::mem;
use std::time::Duration;

pub mod message;

#[derive(Debug)]
pub struct ProtocolError {
    msg: String,
}

impl Display for ProtocolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for ProtocolError {}

impl ProtocolError {
    fn new(msg: impl ToString) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

struct AuroraBus {
    iface: Box<dyn CANInterface>,
    enabled_endpoints: HashMap<u16, EndpointInfo>,
    disabled_endpoints: HashMap<(u16, u16), EndpointInfo>,
}

#[derive(Debug, Copy, Clone)]
pub struct EndpointInfo {
    peripheral_id: u16,
    sequence_no: u16,
    endpoint_id: Option<u16>,
    endpoint_type: EndpointType,
}

impl EndpointInfo {
    pub fn get_fixed_ident(&self) -> (u16, u16) {
        (self.peripheral_id, self.sequence_no)
    }

    pub fn get_type(&self) -> EndpointType {
        self.endpoint_type
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum EndpointType {
    // SAFETY: EndpointType MUST be a unit-only enum (i.e. all variants must have no fields)
    // for the conversion from u32 to be working correctly
    Unknown, // Must be the only non-explict element in the enum
}

impl From<u32> for EndpointType {
    fn from(value: u32) -> Self {
        if value < mem::variant_count::<EndpointType>() as u32 {
            // SAFETY: value is less than the number of variants of EndpointType and thus represents a valid discriminant
            // The cast is only safe if EndpointType is a unit-only enum (i.e. none of the variants have fields
            unsafe {std::mem::transmute(value)}
        } else {
            EndpointType::Unknown
        }
    }
}

impl From<EndpointType> for u32 {
    fn from(value: EndpointType) -> Self {
        value as u32
    }
}

pub trait EndpointConfig {
    fn get_fixed_ident(&self) -> (u16, u16);

    fn get_endpoint_type(&self) -> EndpointType;

    fn configure(&self, iface: &mut dyn CANInterface, info: &EndpointInfo) {}
}

impl AuroraBus {
    pub fn new(iface: impl CANInterface + 'static) -> Self {
        let mut bus = Self {
            iface: Box::new(iface),
            enabled_endpoints: HashMap::new(),
            disabled_endpoints: HashMap::new(),
        };

        bus.initialize();

        bus
    }

    fn initialize(&mut self) {
        // Reset all peripherals
        self.broadcast_reset();

        std::thread::sleep(Duration::from_secs(2));

        // Enumerate endpoints on the bus
        let endpoints = self.enumerate_endpoints(Duration::from_secs_f64(0.1));

        for e in endpoints {
            self.disabled_endpoints
                .insert((e.peripheral_id, e.sequence_no), e);
        }
    }

    pub fn enable_endpoint(&mut self, cfg: &dyn EndpointConfig) -> Result<EndpointInfo> {
        let (peripheral_id, sequence_no) = cfg.get_fixed_ident();

        if let Some(endpoint) = self.disabled_endpoints.get(&(peripheral_id, sequence_no)) {
            if endpoint.endpoint_type != cfg.get_endpoint_type() {
                return Err(anyhow!(
                    "Endpoint type mismatch: Endpoint {:03x}:{:03x} is of type {:?} but was configured as {:?}",
                    peripheral_id,
                    sequence_no,
                    endpoint.endpoint_type,
                    cfg.get_endpoint_type()
                ));
            }
        }

        if let Some(mut endpoint) = self
            .disabled_endpoints
            .remove(&(peripheral_id, sequence_no))
        {
            cfg.configure(&mut *self.iface, &endpoint);

            endpoint.endpoint_id = Some(self.enabled_endpoints.len() as u16);
            if endpoint.endpoint_id.unwrap() > 0x3FF {
                return Err(anyhow!(
                    "Failed to enable endpoint {:?}: Too many endpoints enabled",
                    endpoint
                ));
            }
            self.enabled_endpoints
                .insert(endpoint.endpoint_id.unwrap(), endpoint);

            let enable_msg = Message::new_system_management(
                endpoint.peripheral_id.try_into()?,
                SMMIdent::FlightComputer,
                SMMPayload::EndpointEnable {
                    endpoint_seq_no: endpoint.sequence_no,
                    endpoint_id: endpoint.endpoint_id.unwrap(),
                },
            );

            self.iface.send(enable_msg.into());

            return Ok(endpoint);
        }

        Err(anyhow!(
            "No endpoint found at {:03x}:{:03x}",
            peripheral_id,
            sequence_no
        ))
    }

    pub fn disable_endpoint(&mut self, endpoint_id: u16) -> Result<()> {
        if let Some(mut endpoint) = self.enabled_endpoints.remove(&endpoint_id) {
            endpoint.endpoint_id = None;

            self.disabled_endpoints
                .insert((endpoint.peripheral_id, endpoint.sequence_no), endpoint);

            let disable_msg = Message::new_system_management(
                endpoint.peripheral_id.try_into()?,
                SMMIdent::FlightComputer,
                SMMPayload::EndpointDisable {
                    endpoint_seq_no: endpoint.sequence_no,
                },
            );

            self.iface.send(disable_msg.into());

            Ok(())
        } else {
            Err(anyhow!(
                "No endpoint currently enabled with endpoint ID {:03x}",
                endpoint_id
            ))
        }
    }

    pub fn get_available_endpoints(&self) -> &HashMap<(u16, u16), EndpointInfo> {
        &self.disabled_endpoints
    }

    fn broadcast_reset(&mut self) {
        let reset_msg = Message::new_system_management(
            SMMIdent::Broadcast,
            SMMIdent::FlightComputer,
            SMMPayload::PeripheralReset,
        );

        self.iface.send(reset_msg.into());
    }

    fn enumerate_endpoints(&mut self, timeout: Duration) -> Vec<EndpointInfo> {
        self.enumerate_peripherals(timeout).into_iter().map(|peripheral| {
            let enumerate_msg = Message::new_system_management(
                peripheral,
                SMMIdent::FlightComputer,
                SMMPayload::EndpointDiscoveryRequest
            );

            self.iface.send(enumerate_msg.into());

            let mut endpoints = Vec::new();

            while let Some(resp) = self.iface.recv(Some(timeout)) {
                if let Message::SystemManagement(
                    SystemManagementMessage {
                        receive_identifier,
                        transmit_identifier,
                        payload: SMMPayload::EndpointDiscoveryResponse  {
                            endpoint_count,
                            endpoint_seq_no,
                            endpoint_type
                        }
                    }) = resp.try_into().expect("Failed to decode endpoint discovery response") {
                    assert_eq!(receive_identifier, SMMIdent::FlightComputer);
                    assert_eq!(transmit_identifier, peripheral);

                    endpoints.push(EndpointInfo {
                        peripheral_id: peripheral.into(),
                        sequence_no: endpoint_seq_no,
                        endpoint_id: None,
                        endpoint_type: endpoint_type.into(),
                    });

                    if endpoint_seq_no == endpoint_count - 1 {
                        break;
                    }
                } else {
                    // Unexpected message
                    panic!("Endpoint enumeration failed: Received unexpected message during enumeration phase for peripheral {:?}", peripheral);
                }
            }

            endpoints
        }).flatten().collect()
    }

    fn enumerate_peripherals(&mut self, timeout: Duration) -> Vec<SMMIdent> {
        let enumerate_msg = Message::new_system_management(
            SMMIdent::Broadcast,
            SMMIdent::FlightComputer,
            SMMPayload::PeripheralDiscoveryRequest,
        );

        self.iface.send(enumerate_msg.into());

        let mut peripherals = Vec::new();

        while let Some(resp) = self.iface.recv(Some(timeout)) {
            if let Message::SystemManagement(SystemManagementMessage {
                receive_identifier,
                transmit_identifier,
                payload: SMMPayload::PeripheralDiscoveryResponse,
            }) = resp
                .try_into()
                .expect("Failed to decode peripheral discovery response")
            {
                assert_eq!(receive_identifier, SMMIdent::FlightComputer);

                peripherals.push(transmit_identifier)
            } else {
                // Unexpected message
                panic!("Peripheral enumeration failed: Received unexpected message during enumeration phase");
            }
        }

        peripherals
    }
}
