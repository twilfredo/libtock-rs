//! This sample demonstrates setting up the i2c ip (assuming board has support)
//! for target mode. In the event loop, we first expect the master to write some data
//! then we setup a response packet.
//!
//! NOTE: The device (based on hwip) may stretch clocks by holding the SCL line low if the master attempts to
//! read data before we have setup the read data buffers.
//!
//! This sample is tested with `i2c_master_write_read.rs` sample running on the
//! master device.

#![no_main]
#![no_std]
use core::fmt::Write;
use libmctp::smbus::MCTPSMBusContext;
use libmctp::vendor_packets::VendorIDFormat;
use libtock::alarm::{Alarm, Milliseconds};
use libtock::console::Console;
use libtock::i2c_master_slave::I2CMasterSlave;
use libtock::runtime::{set_main, stack_size};

set_main! {main}
stack_size! {0x1000}

// The address of this device
pub const RESP_ID: u8 = 0x23;
// The address of the requester
pub const REQ_ID: u8 = 0x22;

fn main() {
    let mut request: [u8; 13] = [0; 13];
    let mut response: [u8; 18] = [0; 18];

    assert!(REQ_ID <= 0x7f);
    assert!(RESP_ID <= 0x7f);

    writeln!(Console::writer(), "mctp_resp: setting up\r").unwrap();
    writeln!(Console::writer(), "mctp_resp: address 0x{:x}!\r", RESP_ID).unwrap();

    // Setup MCTP Responder Context
    let msg_types: [u8; 0] = [0; 0];
    let vendor_ids = [VendorIDFormat {
        // PCI Vendor ID
        format: 0x00,
        // PCI VID
        data: 0x1234,
        // Extra data
        numeric_value: 0xAB,
    }];

    let ctx_response = MCTPSMBusContext::new(RESP_ID, &msg_types, &vendor_ids);
    let mut i: u32 = 0;

    loop {
        writeln!(Console::writer(), "mctp_resp: operation {:?}\r", i).unwrap();
        // Expect a write, if the master reads here, the IP may stretch clocks!
        I2CMasterSlave::i2c_master_slave_set_slave_address(RESP_ID)
            .expect("mctp_resp: Failed to listen\r");
        let r = I2CMasterSlave::i2c_master_slave_write_recv_sync(&mut request);
        if let Err(why) = r.1 {
            writeln!(
                Console::writer(),
                "mctp_resp: error to receiving data {:?}\r",
                why
            )
            .unwrap();
        } else {
            // Request received from requester
            writeln!(
                Console::writer(),
                "mctp_resp: {:} bytes received from master \n\r buf: {:x?}\r",
                r.0,
                request
            )
            .unwrap();

            let (_, len) = ctx_response
                .process_packet(&request, &mut response)
                .unwrap();

            writeln!(
                Console::writer(),
                "mctp_resp: generated response: {:x?} - len {:}\r",
                response,
                len.unwrap()
            )
            .unwrap();
            // Allow some time to requester to setup slave
            Alarm::sleep_for(Milliseconds(250)).unwrap();
            let write_len = u16::try_from(len.unwrap()).unwrap();
            // Master bus and write response
            if let Err(why) =
                I2CMasterSlave::i2c_master_slave_write_sync(REQ_ID as u16, &response, write_len)
            {
                writeln!(
                    Console::writer(),
                    "mctp_resp: failed to write response to requester {:?}\r",
                    why
                )
                .unwrap();
            } else {
                // Only iterate loop count on success
                i += 1;
            }
        }
    }
}
