# Aurora CAN Protocol
The Aurora CAN protocol builds on top of CAN FD (CAN Flexible Data-Rate) and is designed to enable plug-and-play use of
peripheral modules.

## Data Types
The following specification uses the following shorthand notations for common numeric data types:
1. U8, U16, U32 and U64 refer to 8 to 64 bit unsigned integers
2. I8, I16, I32 and I64 refer to 8 to 64 bit signed (two's complement) integers
3. F16, F32 and F64 refer to IEEE 754 half-, single- and double precision floating point numbers respectively

Multi-byte types must additionally carry "BE" or "LE" as an endianness suffix, e.g. "U32LE" for a little-endian 32-bit
unsigned integer.

## CAN FD Parameters
The Aurora CAN Protocol uses CAN FD with an arbitration data rate of 1 Mbps (as specified by the CAN FD standard) 
and a payload data rate of 5 Mbps. Use of the Error Status Indicator (ESI) bit is currently reserved and all 
implementations must ignore the ESI bit. The RTR/RTS bit must always be dominant (0). The data length code (DLC) must
be set to accommodate the desired payload length.

### ACK slot
Use of the ACK slot (i.e. which bus nodes should be allowed to ACK messages) is currently undefined.

### CAN IDs
The Aurora CAN protocol uses only 11-bit standard IDs. The use of Extended IDs and the use of the RRS bit as an 
additional ID bit is reserved.

## Terminology
The Aurora CAN protocol governs the communication between multiple hardware units connected to a shared CAN FD bus.
Exactly one of these units is designated the "Flight Computer" and responsible for controlling the behavior of the
other units, designated "Peripheral Units". Each Peripheral Unit consists of one or more logical bus nodes or "Endpoints".

Bus messages are designated as either "System Management Message" (SMM) or "Data Message" (DM), based on the
first (most significant) bit of the CAN ID. If the MSB is dominant (0), the message is a SMM, if it is recessive (1), 
it is a DM.

## Endpoints
Endpoints are classified as either "Data Producer" or "Data Consumer" and should describe a single,
isolated functional units. Data Producer Endpoints produce data (e.g. through measurement of a sensor) in the Peripheral 
Unit and send that data to the Flight Computer for processing. Data Consumer Endpoints receive data (e.g. control target 
values) from the Flight Computer.

In addition to its classification as Data Producer or Data Consumer, each Endpoint is described by a 10-bit ID 
(the "Endpoint ID") and its "Endpoint Type" which governs the format of the payload produced or consumed by the endpoint.

Endpoint IDs are assigned by the Flight Computer on Startup using the Endpoint Discovery and 
Negotiation Process described below. The Aurora CAN protocol allows up to 2^10 = 1024 different Endpoint IDs on a 
single bus.

## Data Messages
The CAN ID of Data Messages consists of a recessive (1) bit in the MSB place and the Endpoint ID. The payload format of 
Data Messages is defined by the Endpoint Type of the respective Endpoint. Data Producer Endpoints may only produce Data Messages
and Data Consumer Endpoints may only consume Data Messages. It is not valid to send DMs using the same Endpoint ID 
in both directions.

## System Management Messages
SMMs are used to configure the Aurora System, discover Endpoints and negotiate Endpoint IDs. For SMMs, the CAN ID consists
of the dominant (0) bit in the MSB place and a fixed, unique 10-bit peripheral unit identifier 
identifying the intended recipient of the message. The all-zeros identifier is reserved
for messages sent to the Flight Computer and the all-ones identifier is reserved for broadcast messages sent from the FC
to all peripherals. Peripheral Units should derive their identifier through means that are fixed across power cycles, e.g.
through hardware serial numbers or fixed firmware constants. Peripheral Units must always listen for SMMs addressed to 
them and to broadcast SMMs.

The SMM payload is structured as follows (from MSB to LSB):
1. 10-bit peripheral identifier of the transmitting unit
2. 6-bit SMM payload length in bytes (excluding the two bytes containing peripheral identifier and length)
3. 8-bit SMM payload type identifier (PTI)
4. Payload data according to payload type identifier

### Currently defined SMM payload types
#### Peripheral Reset (PTI: 0x00)
A peripheral reset command is sent from the FC either as a broadcast or to a specific peripheral unit. Upon reception,
the peripheral unit should perform a reset of its compute unit and any attached hardware and disable all endpoints.
The payload of the reset command is empty.

#### Peripheral Discovery Request (PTI: 0x01)
A peripheral discovery request is sent as a broadcast from the FC to all peripheral units. The payload of the request is empty.
Upon reception, each peripheral unit must answer the request with a Peripheral Discovery Response SMM message.

#### Peripheral Discovery Response (PTI: 0x02)
A peripheral discovery response is sent from peripheral units to the FC upon reception of a Peripheral Discovery Request.
Peripheral Units must retry sending their Peripheral Discovery Response messages until successful, to ensure delivery
of all responses. The payload of the Peripheral Discovery Response is empty.

#### Endpoint Discovery Request (PTI: 0x03)
An endpoint discovery request is sent from the FC to a specific peripheral unit. Upon reception, the peripheral unit must
answer with one or more Endpoint Discovery Responses. The payload of the request is empty.

#### Endpoint Discovery Response (PTI: 0x04)
Upon reception of an Endpoint Discovery Request, a peripheral unit must enumerate its endpoints by sending one
Endpoint Discovery Response per endpoint. The payload of the Endpoint Discovery Response has the following format:
1. Number of endpoints on this peripheral unit (U16LE)
2. Sequence number of the current endpoint (U16LE)
3. Endpoint Type Identifier (U32LE)

#### Endpoint Enable (PTI: 0x05)
To enable an endpoint, the Flight Computer sends an Endpoint Enable Message to a specific peripheral unit. A peripheral
unit must only produce or receive data for enabled endpoints. All endpoints are disabled on startup/reset. The payload
of the Endpoint Enable Message has the following format:
1. Sequence number of the endpoint to enable (U16LE)
2. Endpoint ID to be used by the endpoint (U16LE)

#### Endpoint Disable (PTI: 0x06)
To disable an endpoint, the Flight Computer sends an Endpoint Disable Message to a specific peripheral unit.
A peripheral unit must not continue producing or accepting endpoint messages for disabled endpoints. If the flight 
computer disables an endpoint and then reassigns the Endpoint ID of the disabled Endpoint to a different Endpoint,
it is the responsibility of the Flight Computer to ensure that any still pending messages from the old endpoint are not
misinterpreted. The payload of the Endpoint Disable Message has the following format:
1. Sequence number (not Endpoint ID!) of the endpoint to disable (U16LE)

#### Endpoint Configuration Read Request (PTI: 0x07)
Endpoints may expose a dictionary of configuration options that tune their behavior. Each configuration item is
identified by a 32-bit identifier and has a 64-bit value. Configuration items may be read-only, write-only or read-write.
Configuration items may be altered for enabled and for disabled endpoints. The behavior for altering configuration items
of enabled endpoints may be erratic, depending on the endpoint.
An Endpoint Configuration Read Request is sent from the Flight Computer to a peripheral unit in order to read configuration items.
The payload of the Endpoint Configuration Read Request has the following format:
1. Sequence number (not Endpoint ID!) of the endpoint (U16LE)
2. Configuration item identifier (U32LE)

#### Endpoint Configuration Read Response (PTI: 0x08)
Upon reception of an Endpoint Configuration Read Request, a peripheral unit must respond with an Endpoint Configuration Read Response.
The payload of the Endpoint Configuration Read Response has the following format:
1. Sequence number (not Endpoint ID!) of the endpoint (U16LE)
2. Configuration item identifier (U32LE)
3. Flags (U8, from MSB to LSB: SCXX XXXX, S indicates status: 0 is a success, 1 is an error (invalid or write-only item), 
C indicates whether the configuration item has changed since the last reset: 0 if no change, 1 otherwise, X indicates a
reserved bit which should be ignored by all implementations)
4. Configuration item value (U64LE, actual interpretation of the value is implementation-defined)

#### Endpoint Configuration Write (PTI: 0x09)
An Endpoint Configuration Write is sent by the flight computer to a specific peripheral unit in order to set the value
of a specific configuration item. The payload of an Endpoint Configuration Write has the following format:
1. Sequence number (not Endpoint ID!) of the endpoint (U16LE)
2. Configuration item identifier (U32LE)
3. Configuration item value (U64LE, actual interpretation of the value is implementation-defined)

Writes to invalid (non-existent or read-only) configuration items may be silently ignored or trigger an error condition.

#### Peripheral Fault Message (PTI: 0x0A)
Peripheral units may report faults to the flight computer using a Peripheral Fault Message. After sending a Peripheral
Fault Message, a peripheral must perform a reset, if possible. The payload of a Peripheral Fault Message is a null-terminated, 
UTF-8 encoded string.

#### Peripheral Locator Set/Reset (PTI: 0x0B)
Peripheral units may feature a locating device (e.g. an LED) to help identify the peripheral unit during initial commissioning.
The flight computer may send a Peripheral Locator Set/Reset command to enable or disable the locating device on a particular
peripheral unit. The command is also valid (although possibly less useful) when sent as a broadcast. 
The payload of the command is a single byte. Any non-zero value is treated as an "Enable" command and a zero byte is
treated as a "Disable" command for the locating device.