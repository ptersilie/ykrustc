// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_type = "bin"]
#![feature(lang_items)]
#![no_main]
#![no_std]

use core::alloc::Layout;
use core::panic::PanicInfo;

#[lang = "yk_swt_rec_loc"]
fn yk_swt_rec_loc(_crate_hash: u64, _def_idx: u32, _bb: u32) {}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
fn eh() {}

#[lang = "oom"]
fn oom(_: Layout) -> ! {
    loop {}
}
