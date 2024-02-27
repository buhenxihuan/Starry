#![no_std]
#![no_main]
// extern crate axstarry;

use syscall_entry::run_testcases;

#[no_mangle]
fn main() {
    let tc = option_env!("AX_TC").unwrap_or("libc-static");
    run_testcases(tc);
}
