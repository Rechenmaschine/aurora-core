use crate::can::protocol::message::{Message, SMMIdent, SMMPayload, SystemManagementMessage};
use crate::can::CANInterface;
use std::time::Duration;

pub mod message;

#[derive(Debug)]
pub struct ProtocolError {
    msg: String,
}

impl ProtocolError {
    fn new(msg: impl ToString) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

struct AuroraBus {
    iface: Box<dyn CANInterface>,
    known_endpoints: Vec<EndpointInfo>,
}

pub struct EndpointInfo {
    peripheral_id: u16,
    sequence_no: u16,
    endpoint_id: Option<u16>,
    endpoint_type: EndpointType,
}

pub enum EndpointType {
    Unknown,
}

impl From<u32> for EndpointType {
    fn from(value: u32) -> Self {
        match value {
            _ => EndpointType::Unknown,
        }
    }
}

impl AuroraBus {
    pub fn new(iface: impl CANInterface + 'static) -> Self {
        Self {
            iface: Box::new(iface),
            known_endpoints: Vec::new(),
        }
    }

    pub fn initialize(&mut self) {
        // Reset all peripherals
        self.broadcast_reset();

        std::thread::sleep(Duration::from_secs(2));

        // Enumerate endpoints on the bus
        self.known_endpoints = self.enumerate_endpoints(Duration::from_secs_f64(0.1));

        // TODO: Configure endpoints and assign endpoint IDs
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
