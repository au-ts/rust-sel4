//
// Copyright 2025, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

use core::ops::Range;
use core::ptr;
use core::slice;

use rkyv::Archive;

use sel4_capdl_initializer_types::SpecForInitializer;
use sel4_immutable_cell::ImmutableCell;
use sel4_logging::{LevelFilter, Logger, LoggerBuilder};
use sel4_root_task::{debug_print, root_task};
use sel4::{UntypedDesc, sel4_cfg_usize};

use crate::initialize::Initializer;

#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".data")]
static sel4_capdl_initializer_serialized_spec_data_start: ImmutableCell<*mut u8> =
    ImmutableCell::new(ptr::null_mut());

#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".data")]
static sel4_capdl_initializer_serialized_spec_data_size: ImmutableCell<usize> =
    ImmutableCell::new(0);

#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".data")]
static sel4_capdl_initializer_embedded_frames_data_start: ImmutableCell<*mut u8> =
    ImmutableCell::new(ptr::null_mut());

#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".data")]
static sel4_capdl_initializer_image_start: ImmutableCell<*mut u8> =
    ImmutableCell::new(ptr::null_mut());

#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".data")]
static sel4_capdl_initializer_image_end: ImmutableCell<*mut u8> =
    ImmutableCell::new(ptr::null_mut());

const UNTYPED_DESC_SIZE: usize = size_of::<UntypedDesc>();
const MAX_UNTYPEDS: usize = sel4_cfg_usize!(MAX_NUM_BOOTINFO_UNTYPED_CAPS);

/// Optional
#[unsafe(no_mangle)]
#[unsafe(link_section = ".data")]
static mut sel4_capdl_initializer_expected_untypeds_list: [u8; UNTYPED_DESC_SIZE * MAX_UNTYPEDS] =
    [0; UNTYPED_DESC_SIZE * MAX_UNTYPEDS];

/// Optional
#[unsafe(no_mangle)]
#[unsafe(link_section = ".data")]
static mut sel4_capdl_initializer_expected_untypeds_list_num_entries: usize = 0;

const LOG_LEVEL: LevelFilter = {
    // LevelFilter::Trace
    // LevelFilter::Debug
    LevelFilter::Info
};

static LOGGER: Logger = LoggerBuilder::const_default()
    .level_filter(LOG_LEVEL)
    .filter(|meta| meta.target().starts_with("sel4_capdl_initializer"))
    .write(|s| debug_print!("{}", s))
    .build();

#[cfg_attr(
    feature = "alloc",
    root_task(stack_size = 0x10_000, heap_size = 0x10_000)
)]
#[cfg_attr(not(feature = "alloc"), root_task(stack_size = 0x10_000))]
fn main(bootinfo: &sel4::BootInfoPtr) -> ! {
    LOGGER.set().unwrap();
    let spec = access_spec(get_spec_bytes());
    Initializer::initialize(
        bootinfo,
        user_image_bounds(),
        spec,
        *sel4_capdl_initializer_embedded_frames_data_start.get() as usize,
        get_expected_untypeds(),
    )
}

fn get_spec_bytes() -> &'static [u8] {
    unsafe {
        slice::from_raw_parts(
            *sel4_capdl_initializer_serialized_spec_data_start.get(),
            *sel4_capdl_initializer_serialized_spec_data_size.get(),
        )
    }
}

fn user_image_bounds() -> Range<usize> {
    (*sel4_capdl_initializer_image_start.get() as usize)
        ..(*sel4_capdl_initializer_image_end.get() as usize)
}

#[cfg(feature = "alloc")]
fn access_spec(bytes: &[u8]) -> &<SpecForInitializer as Archive>::Archived {
    SpecForInitializer::access(bytes).unwrap()
}

#[cfg(not(feature = "alloc"))]
fn access_spec(bytes: &[u8]) -> &<SpecForInitializer as Archive>::Archived {
    unsafe { SpecForInitializer::access_unchecked(bytes) }
}

/// This is useful for error checking when your upstream spec generation tool expects a certain range of untypeds from the kernel.
fn get_expected_untypeds() -> &'static [UntypedDesc] {
    #[allow(static_mut_refs)]
    unsafe {
        let num_entries = sel4_capdl_initializer_expected_untypeds_list_num_entries
            .min(sel4::sel4_cfg_usize!(MAX_NUM_BOOTINFO_UNTYPED_CAPS));
        let ptr = sel4_capdl_initializer_expected_untypeds_list.as_ptr() as *const UntypedDesc;
        core::slice::from_raw_parts(ptr, num_entries)
    }
}
