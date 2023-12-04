use crate::can::protocol::ProtocolError;
use crate::can::{CANFDFrame, CANFDPayload};
use arrayvec::ArrayString;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Message {
    Data(DataMessage),
    SystemManagement(SystemManagementMessage),
}

impl Message {
    pub fn new_system_management(recv: SMMIdent, send: SMMIdent, payload: SMMPayload) -> Self {
        Self::SystemManagement(SystemManagementMessage {
            receive_identifier: recv,
            transmit_identifier: send,
            payload,
        })
    }
}

const CAN_ID_MSB: u16 = 1 << 10;

impl From<Message> for CANFDFrame {
    fn from(msg: Message) -> Self {
        match msg {
            Message::Data(dm) => {
                assert!(dm.endpoint_id < 1024);

                CANFDFrame {
                    id: dm.endpoint_id | CAN_ID_MSB,
                    payload: dm.payload,
                }
            }
            Message::SystemManagement(smm) => CANFDFrame {
                id: smm.receive_identifier.into(),
                payload: smm.encode_as_can_payload(),
            },
        }
    }
}

impl TryFrom<CANFDFrame> for Message {
    type Error = ProtocolError;

    fn try_from(frame: CANFDFrame) -> Result<Self, Self::Error> {
        match frame.id {
            0x000..=0x3FF => {
                // System Management Message
                let recv_id = frame.id;

                match frame.payload.len() {
                    3.. => {
                        let payload_header =
                            u16::from_be_bytes(frame.payload[..2].try_into().unwrap());
                        let transmit_id = payload_header >> 6;
                        let payload_length = payload_header & 0b0011_1111;
                        let payload_type_ident = frame.payload[2];

                        if frame.payload.len() < 2 + payload_length as usize {
                            return Err(ProtocolError::new(
                                "Failed SMM decode: Invalid payload length mismatch",
                            ));
                        }

                        let payload = SMMPayload::decode(
                            payload_type_ident,
                            &frame.payload[3..(2 + payload_length as usize)],
                        )?;

                        Ok(Self::new_system_management(
                            recv_id.try_into()?,
                            transmit_id.try_into()?,
                            payload,
                        ))
                    }
                    _ => Err(ProtocolError::new(
                        "Invalid payload length for a system management message",
                    )),
                }
            }
            0x400..=0x7FF => {
                // Data Message
                Ok(Self::Data(DataMessage {
                    endpoint_id: frame.id ^ CAN_ID_MSB, // Clear the ID MSB
                    payload: frame.payload,
                }))
            }
            _ => Err(ProtocolError::new("Invalid CAN ID")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]

pub struct DataMessage {
    pub endpoint_id: u16,
    pub payload: CANFDPayload,
}

#[derive(Clone, Debug, PartialEq, Eq)]

pub struct SystemManagementMessage {
    pub receive_identifier: SMMIdent,
    pub transmit_identifier: SMMIdent,
    pub payload: SMMPayload,
}

impl SystemManagementMessage {
    fn encode_as_can_payload(&self) -> CANFDPayload {
        let mut buf = CANFDPayload::new();
        buf.try_extend_from_slice(&[0, 0]).unwrap();

        let transmit_ident: u16 = self.transmit_identifier.into();
        self.payload.encode_into(&mut buf);
        let header: [u8; 2] = ((transmit_ident << 6) | (buf.len() - 2) as u16).to_be_bytes();
        buf[0..2].copy_from_slice(&header);

        buf
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SMMIdent {
    FlightComputer,
    Broadcast,
    Peripheral(u16),
}

impl From<SMMIdent> for u16 {
    fn from(value: SMMIdent) -> Self {
        match value {
            SMMIdent::FlightComputer => 0,
            SMMIdent::Broadcast => 0x3FF,
            SMMIdent::Peripheral(v) => {
                assert!(v > 0 && v < 0x3FF);
                v
            }
        }
    }
}

impl TryFrom<u16> for SMMIdent {
    type Error = ProtocolError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::FlightComputer),
            0x3FF => Ok(Self::Broadcast),
            1..=0x3FE => Ok(Self::Peripheral(value)),
            _ => Err(ProtocolError::new("Invalid SMM Identifier")),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]

pub enum SMMPayload {
    PeripheralReset,
    PeripheralDiscoveryRequest,
    PeripheralDiscoveryResponse,
    EndpointDiscoveryRequest,
    EndpointDiscoveryResponse {
        endpoint_count: u16,
        endpoint_seq_no: u16,
        endpoint_type: u32,
    },
    EndpointEnable {
        endpoint_seq_no: u16,
        endpoint_id: u16,
    },
    EndpointDisable {
        endpoint_seq_no: u16,
    },
    EndpointConfigurationReadRequest {
        endpoint_seq_no: u16,
        config_item_no: u32,
    },
    EndpointConfigurationReadResponse {
        endpoint_seq_no: u16,
        config_item_no: u32,
        error: bool,
        has_changed: bool,
        config_item_value: u64,
    },
    EndpointConfigurationWrite {
        endpoint_seq_no: u16,
        config_item_ident: u32,
        config_item_value: u64,
    },
    PeripheralFaultMessage {
        msg: ArrayString<60>,
    },
    PeripheralLocatorSetReset {
        locator_on: bool,
    },
}

impl SMMPayload {
    fn encode_into(&self, buf: &mut CANFDPayload) {
        match self {
            SMMPayload::PeripheralReset => {
                buf.push(0x00);
            }
            SMMPayload::PeripheralDiscoveryRequest => {
                buf.push(0x01);
            }
            SMMPayload::PeripheralDiscoveryResponse => {
                buf.push(0x02);
            }
            SMMPayload::EndpointDiscoveryRequest => {
                buf.push(0x03);
            }
            SMMPayload::EndpointDiscoveryResponse {
                endpoint_count,
                endpoint_seq_no,
                endpoint_type,
            } => {
                buf.push(0x04);
                buf.try_extend_from_slice(&endpoint_count.to_le_bytes())
                    .unwrap();
                buf.try_extend_from_slice(&endpoint_seq_no.to_le_bytes())
                    .unwrap();
                buf.try_extend_from_slice(&endpoint_type.to_le_bytes())
                    .unwrap();
            }
            SMMPayload::EndpointEnable {
                endpoint_seq_no,
                endpoint_id,
            } => {
                buf.push(0x05);
                buf.try_extend_from_slice(&endpoint_seq_no.to_le_bytes())
                    .unwrap();
                buf.try_extend_from_slice(&endpoint_id.to_le_bytes())
                    .unwrap();
            }
            SMMPayload::EndpointDisable { endpoint_seq_no } => {
                buf.push(0x06);
                buf.try_extend_from_slice(&endpoint_seq_no.to_le_bytes())
                    .unwrap();
            }
            SMMPayload::EndpointConfigurationReadRequest {
                endpoint_seq_no,
                config_item_no,
            } => {
                buf.push(0x07);
                buf.try_extend_from_slice(&endpoint_seq_no.to_le_bytes())
                    .unwrap();
                buf.try_extend_from_slice(&config_item_no.to_le_bytes())
                    .unwrap();
            }
            SMMPayload::EndpointConfigurationReadResponse {
                endpoint_seq_no,
                config_item_no,
                error,
                has_changed,
                config_item_value,
            } => {
                buf.push(0x08);
                buf.try_extend_from_slice(&endpoint_seq_no.to_le_bytes())
                    .unwrap();
                buf.try_extend_from_slice(&config_item_no.to_le_bytes())
                    .unwrap();
                let flags =
                    (if *error { 1 << 7 } else { 0 }) | (if *has_changed { 1 << 6 } else { 0 });
                buf.push(flags);
                buf.try_extend_from_slice(&config_item_value.to_le_bytes())
                    .unwrap();
            }
            SMMPayload::EndpointConfigurationWrite {
                endpoint_seq_no,
                config_item_ident,
                config_item_value,
            } => {
                buf.push(0x09);
                buf.try_extend_from_slice(&endpoint_seq_no.to_le_bytes())
                    .unwrap();
                buf.try_extend_from_slice(&config_item_ident.to_le_bytes())
                    .unwrap();
                buf.try_extend_from_slice(&config_item_value.to_le_bytes())
                    .unwrap();
            }
            SMMPayload::PeripheralFaultMessage { msg } => {
                buf.push(0x0A);
                buf.try_extend_from_slice(msg.as_bytes()).unwrap();
                buf.push(0x00);
            }
            SMMPayload::PeripheralLocatorSetReset { locator_on } => {
                buf.push(0x0B);
                buf.push(if *locator_on { 1 } else { 0 })
            }
        }
    }

    fn decode(pti: u8, payload_data: &[u8]) -> Result<Self, ProtocolError> {
        match pti {
            0x00 => {
                if payload_data.len() == 0 {
                    Ok(SMMPayload::PeripheralReset)
                } else {
                    Err(ProtocolError::new(
                        "Failed to decode SMM payload: Invalid payload length for Peripheral Reset",
                    ))
                }
            }
            0x01 => {
                if payload_data.len() == 0 {
                    Ok(SMMPayload::PeripheralDiscoveryRequest)
                } else {
                    Err(ProtocolError::new("Failed to decode SMM payload: Invalid payload length for Peripheral Discovery Request"))
                }
            }
            0x02 => {
                if payload_data.len() == 0 {
                    Ok(SMMPayload::PeripheralDiscoveryResponse)
                } else {
                    Err(ProtocolError::new("Failed to decode SMM payload: Invalid payload length for Peripheral Discovery Response"))
                }
            }
            0x03 => {
                if payload_data.len() == 0 {
                    Ok(SMMPayload::EndpointDiscoveryRequest)
                } else {
                    Err(ProtocolError::new("Failed to decode SMM payload: Invalid payload length for Endpoint Discovery Request"))
                }
            }
            0x04 => {
                if payload_data.len() == 8 {
                    Ok(SMMPayload::EndpointDiscoveryResponse {
                        endpoint_count: u16::from_le_bytes(payload_data[..2].try_into().unwrap()),
                        endpoint_seq_no: u16::from_le_bytes(payload_data[2..4].try_into().unwrap()),
                        endpoint_type: u32::from_le_bytes(payload_data[4..].try_into().unwrap()),
                    })
                } else {
                    Err(ProtocolError::new("Failed to decode SMM payload: Invalid payload length for Endpoint Discovery Response"))
                }
            }
            0x05 => {
                if payload_data.len() == 4 {
                    Ok(SMMPayload::EndpointEnable {
                        endpoint_seq_no: u16::from_le_bytes(payload_data[..2].try_into().unwrap()),
                        endpoint_id: u16::from_le_bytes(payload_data[2..].try_into().unwrap()),
                    })
                } else {
                    Err(ProtocolError::new(
                        "Failed to decode SMM payload: Invalid payload length for Endpoint Enable",
                    ))
                }
            }
            0x06 => {
                if payload_data.len() == 2 {
                    Ok(SMMPayload::EndpointDisable {
                        endpoint_seq_no: u16::from_le_bytes(payload_data[..2].try_into().unwrap()),
                    })
                } else {
                    Err(ProtocolError::new(
                        "Failed to decode SMM payload: Invalid payload length for Endpoint Disable",
                    ))
                }
            }
            0x07 => {
                if payload_data.len() == 6 {
                    Ok(SMMPayload::EndpointConfigurationReadRequest {
                        endpoint_seq_no: u16::from_le_bytes(payload_data[..2].try_into().unwrap()),
                        config_item_no: u32::from_le_bytes(payload_data[2..].try_into().unwrap()),
                    })
                } else {
                    Err(ProtocolError::new("Failed to decode SMM payload: Invalid payload length for Endpoint Configuration Read Request"))
                }
            }
            0x08 => {
                if payload_data.len() == 15 {
                    Ok(SMMPayload::EndpointConfigurationReadResponse {
                        endpoint_seq_no: u16::from_le_bytes(payload_data[..2].try_into().unwrap()),
                        config_item_no: u32::from_le_bytes(payload_data[2..6].try_into().unwrap()),
                        error: payload_data[6] & 0b1000_0000 != 0,
                        has_changed: payload_data[6] & 0b0100_0000 != 0,
                        config_item_value: u64::from_le_bytes(
                            payload_data[7..].try_into().unwrap(),
                        ),
                    })
                } else {
                    Err(ProtocolError::new("Failed to decode SMM payload: Invalid payload length for Endpoint Configuration Read Response"))
                }
            }
            0x09 => {
                if payload_data.len() == 14 {
                    Ok(SMMPayload::EndpointConfigurationWrite {
                        endpoint_seq_no: u16::from_le_bytes(payload_data[..2].try_into().unwrap()),
                        config_item_ident: u32::from_le_bytes(
                            payload_data[2..6].try_into().unwrap(),
                        ),
                        config_item_value: u64::from_le_bytes(
                            payload_data[6..].try_into().unwrap(),
                        ),
                    })
                } else {
                    Err(ProtocolError::new("Failed to decode SMM payload: Invalid payload length for Endpoint Configuration Write"))
                }
            }
            0x0A => {
                if let Some(0x00) = payload_data.last() {
                    if let Ok(msg) = std::str::from_utf8(&payload_data[..payload_data.len() - 1]) {
                        if let Ok(msg) = ArrayString::from_str(msg) {
                            Ok(SMMPayload::PeripheralFaultMessage { msg })
                        } else {
                            Err(ProtocolError::new(
                                "Failed to decode SMM payload: Payload too long",
                            ))
                        }
                    } else {
                        Err(ProtocolError::new(
                            "Failed to decode SMM payload: Payload not valid UTF-8",
                        ))
                    }
                } else {
                    Err(ProtocolError::new(
                        "Failed to decode SMM payload: Payload not null-terminated",
                    ))
                }
            }
            0x0B => {
                if payload_data.len() == 1 {
                    Ok(SMMPayload::PeripheralLocatorSetReset {
                        locator_on: payload_data[0] != 0,
                    })
                } else {
                    Err(ProtocolError::new("Failed to decode SMM payload: Invalid payload length for Peripheral Locator Set/Reset"))
                }
            }
            _ => Err(ProtocolError::new(
                "Failed to decode SMM payload: Unknown Payload Type Identifier",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding_roundtrip() {
        let test_message = Message::SystemManagement(SystemManagementMessage {
            receive_identifier: SMMIdent::Peripheral(42),
            transmit_identifier: SMMIdent::FlightComputer,
            payload: SMMPayload::EndpointConfigurationReadRequest {
                endpoint_seq_no: 0,
                config_item_no: 0xBADEAFFE,
            },
        });

        let frame = CANFDFrame::from(test_message.clone());

        let decoded = Message::try_from(frame).unwrap();
        assert_eq!(test_message, decoded);
    }
}
