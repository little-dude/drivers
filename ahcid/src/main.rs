#![feature(asm)]

#[macro_use]
extern crate bitflags;
extern crate io;
extern crate syscall;

use std::{env, thread, usize};

use syscall::{iopl, physmap, physunmap, MAP_WRITE};

pub mod ahci;

fn main() {
    let mut args = env::args().skip(1);

    let bar_str = args.next().expect("ahcid: no address provided");
    let bar = usize::from_str_radix(&bar_str, 16).expect("ahcid: failed to parse address");

    let irq_str = args.next().expect("ahcid: no irq provided");
    let irq = irq_str.parse::<u8>().expect("ahcid: failed to parse irq");

    thread::spawn(move || {
        unsafe {
            iopl(3).expect("ahcid: failed to get I/O permission");
            asm!("cli" :::: "intel", "volatile");
        }

        let address = unsafe { physmap(bar, 4096, MAP_WRITE).expect("ahcid: failed to map address") };
        {
            ahci::Ahci::disks(address, irq);
        }
        unsafe { physunmap(address).expect("ahcid: failed to unmap address") };
    });
}