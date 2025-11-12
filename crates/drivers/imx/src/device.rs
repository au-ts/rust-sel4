//
// Copyright 2025, UNSW
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![allow(dead_code)]

use core::ops::Deref;

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_structs,
    registers::ReadWrite,
};

const UART_STAT_TDRE: u32 = 1 << 14;

register_structs! {
    #[allow(non_snake_case)]
    pub(crate)ImxRegisterBlock {
        (0x00 => _reserved0),
        (0x40 => transmit: ReadWrite<u32>),
        (0x44 => _reserved1),
        (0x98 => stat: ReadWrite<u32>),
        (0x9c => @END),
    }
}

pub(crate) struct Device {
    ptr: *mut ImxRegisterBlock,
}

impl Device {
    pub(crate) const unsafe fn new(ptr: *mut ImxRegisterBlock) -> Self {
        Self { ptr }
    }

    fn ptr(&self) -> *const ImxRegisterBlock {
        self.ptr
    }

    pub(crate) fn init(&self) {

    }

    pub(crate) fn put_char(&self, c: u8) {
        while (self.stat.get() & UART_STAT_TDRE) == 0 {}
        self.transmit.set(c as u32);
    }
}

impl Deref for Device {
    type Target = ImxRegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}