pub mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    use bytemuck::{Pod, Zeroable};

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    unsafe impl Zeroable for rfkill_event {}
    unsafe impl Pod for rfkill_event {}
}

use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
    os::unix::prelude::OpenOptionsExt,
};

use bytemuck::{bytes_of, bytes_of_mut};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use sys::*;

const ZERO_RFKILL_EVENT: rfkill_event = rfkill_event {
    idx: 0,
    type_: 0,
    op: 0,
    soft: 0,
    hard: 0,
};

fn new_event_by_type(block: bool, type_: rfkill_type) -> rfkill_event {
    rfkill_event {
        op: rfkill_operation_RFKILL_OP_CHANGE_ALL as u8,
        type_: type_ as u8,
        soft: block as u8,
        ..ZERO_RFKILL_EVENT
    }
}

fn new_event_by_index(block: bool, index: u32) -> rfkill_event {
    rfkill_event {
        op: rfkill_operation_RFKILL_OP_CHANGE as u8,
        idx: index,
        soft: block as u8,
        ..ZERO_RFKILL_EVENT
    }
}

fn write_event(event: &rfkill_event) -> io::Result<()> {
    let mut dev = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/rfkill")?;
    dev.write(bytes_of(event))?;

    Ok(())
}

pub fn block_index(block: bool, index: u32) -> io::Result<()> {
    let event = new_event_by_index(block, index);
    write_event(&event)
}

#[derive(Debug, Clone, Copy, FromPrimitive)]
#[repr(u32)]
pub enum RfkillType {
    All = rfkill_type_RFKILL_TYPE_ALL,
    Wlan = rfkill_type_RFKILL_TYPE_WLAN,
    Bluetooth = rfkill_type_RFKILL_TYPE_BLUETOOTH,
    Uwb = rfkill_type_RFKILL_TYPE_UWB,
    Wimax = rfkill_type_RFKILL_TYPE_WIMAX,
    WWan = rfkill_type_RFKILL_TYPE_WWAN,
    Gps = rfkill_type_RFKILL_TYPE_GPS,
    Fm = rfkill_type_RFKILL_TYPE_FM,
    Nfc = rfkill_type_RFKILL_TYPE_NFC,
}

#[derive(Clone, Debug)]
pub struct RfkillEvent {
    pub idx: u32,
    pub type_: RfkillType,
    pub soft: bool,
    pub hard: bool,
}

impl From<rfkill_event> for RfkillEvent {
    fn from(event: rfkill_event) -> Self {
        RfkillEvent {
            idx: event.idx,
            type_: RfkillType::from_u8(event.type_).unwrap(),
            soft: event.soft != 0,
            hard: event.hard != 0,
        }
    }
}

pub fn block_type(block: bool, type_: RfkillType) -> io::Result<()> {
    let event = new_event_by_type(block, type_ as u32);
    write_event(&event)
}

pub fn list() -> io::Result<Vec<RfkillEvent>> {
    let mut dev = OpenOptions::new()
        .read(true)
        .write(false)
        .custom_flags(libc::O_NONBLOCK)
        .open("/dev/rfkill")?;
    let mut events = vec![];
    loop {
        let mut event = ZERO_RFKILL_EVENT;
        match dev.read_exact(bytes_of_mut(&mut event)) {
            Ok(()) => events.push(event.into()),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
            e @ Err(_) => e?,
        }
    }
    Ok(events)
}
