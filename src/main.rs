#![no_std]
#![no_main]

use riscvp2_rt::*;

#[no_mangle]
pub fn main() -> ! {
    loop {
      pintoggle(57);
      waitms(200);
    }
}
