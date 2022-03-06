#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};

#[entry]
fn main() -> ! {    
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let light_all = [
        [0,1,0,1,0],
        [1,1,1,1,1],
        [1,1,1,1,1],
        [0,1,1,1,0],
        [0,0,1,0,0]
    ];

    loop {
        display.show(&mut timer, light_all, 1000);
        display.clear();
        timer.delay_ms(1000_u16);
    }
}