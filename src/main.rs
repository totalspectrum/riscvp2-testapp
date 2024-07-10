#![no_std]
#![no_main]

use riscvp2_rt::*;

#[no_mangle]
pub fn main() -> ! {
    let mut ch : u8 = b'A';
    loop {
      pintoggle(57);
      waitms(200);
      tx(ch);
      ch = ch + 1;
      if ch > b'Z' {
        ch = b'A';
      }
    }
}
