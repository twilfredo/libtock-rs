//! Sample application to use libmctp as the requester. This will setup a
//! requester context, then send a simple request to the responder over I2C.
//! Then assert that the response is valid.

#![no_main]
#![no_std]
use core::fmt::Write;
use libmctp::base_packet::*;
use libmctp::control_packet::MCTPVersionQuery;
use libmctp::control_packet::*;
use libmctp::smbus::MCTPSMBusContext;
use libmctp::vendor_packets::VendorIDFormat;
use libtock::alarm::{Alarm, Milliseconds};
use libtock::console::Console;
use libtock::i2c_master_slave::I2CMasterSlave;
use libtock::runtime::{set_main, stack_size};

set_main! {main}
stack_size! {0x1000}
// The address of this device
pub const REQ_ID: u8 = 0x22;
// The address of the responder
pub const RESP_ID: u8 = 0x23;

// Switch into slave/target mode, and expect the responder to master the bus
// then write the response back to us.
fn expect_response(response: &mut [u8]) -> Result<usize, ()> {
    assert!(response.len() >= 32);
    // Setup slave mode
    I2CMasterSlave::i2c_master_slave_set_slave_address(REQ_ID)
        .expect("mctp-requester: Failed to listen");
    let r = I2CMasterSlave::i2c_master_slave_write_recv_sync(response);

    if let Err(why) = r.1 {
        writeln!(
            Console::writer(),
            "mctp-requester: error to receiving data {:?}\r",
            why
        )
        .unwrap();
        return Err(());
    }

    writeln!(
        Console::writer(),
        "{:} bytes received from responder | buf: {:x?}\r",
        r.0,
        response
    )
    .unwrap();

    Ok(r.0)
}

fn assert_mctp_version_support_resonse(len: usize, response: &[u8]) {
    assert_eq!(len, 18);

    // Destination address
    assert_eq!(response[0], REQ_ID << 1);

    // Byte count
    assert_eq!(response[2], 14);

    // IC and Message Type
    assert_eq!(response[8], 0 << 7 | MessageType::MCtpControl as u8);
    // Rq, D, rsvd and Instance ID
    assert_eq!(response[9], 0 << 7 | 0 << 6 | 0 << 5 | 0);

    // Command Code
    assert_eq!(response[10], CommandCode::GetMCTPVersionSupport as u8);
    // Completion Code
    assert_eq!(response[11], CompletionCode::Success as u8);

    // Version Entry Count
    assert_eq!(response[12], 1);
    // Major version number
    assert_eq!(response[13], 0xF1);
    // Major version number
    assert_eq!(response[14], 0xF3);
    // Update version number
    assert_eq!(response[15], 0xF1);
    // Alpha byte
    assert_eq!(response[16], 0x00);

    writeln!(
        Console::writer(),
        "mctp-requester: mctp_version_support_resonse assertion .... [OK]\r"
    )
    .unwrap();
}

fn main() {
    assert!(REQ_ID <= 0x7f);
    assert!(RESP_ID <= 0x7f);
    writeln!(Console::writer(), "mctp-requester sample\r").unwrap();
    // Setup and MCTP requester context and generate an arbitrary request.
    let msg_types: [u8; 0] = [0; 0];
    // Specify a PCI vendor ID that we support
    let vendor_ids = [VendorIDFormat {
        // PCI Vendor ID
        format: 0x00,
        // PCI VID
        data: 0x1234,
        // Extra data
        numeric_value: 0xAB,
    }];

    let ctx = MCTPSMBusContext::new(REQ_ID, &msg_types, &vendor_ids);
    let mut request: [u8; 13] = [0; 13];
    let mut response: [u8; 32] = [0; 32];
    let len = ctx
        .get_request()
        .get_mctp_version_support(RESP_ID, MCTPVersionQuery::MCTPBaseSpec, &mut request)
        .expect("mctp-requester: failed to generate get mctp version support request\r");
    let len: u16 = u16::try_from(len).unwrap();

    loop {
        // Send Responder
        writeln!(
            Console::writer(),
            "mctp-requester: i2c: sending request || {:x?}\r",
            request
        )
        .unwrap();
        if let Err(why) = I2CMasterSlave::i2c_master_slave_write_sync(RESP_ID as u16, &request, len)
        {
            writeln!(
                Console::writer(),
                "mctp-requester: i2c: write operation failed {:?}\r",
                why
            )
            .unwrap();
        } else {
            // Write was a success, now wait for a response in slave mode.
            writeln!(Console::writer(), "mctp-requester: waiting for response\r").unwrap();
            // Expect Response from responder
            let recv_len = expect_response(&mut response)
                .expect("mctp-requester: failed to receive response\r");
            writeln!(
                Console::writer(),
                "mctp-requester: response {:x?}\r",
                response
            )
            .unwrap();
            // Assert correctness in the response buffer
            assert_mctp_version_support_resonse(recv_len, &response);
        }

        // Delay then re-run
        Alarm::sleep_for(Milliseconds(1000)).unwrap();
    }
}
