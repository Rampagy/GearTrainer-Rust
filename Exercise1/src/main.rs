#![no_std]
#![no_main]


use cortex_m_rt::entry;
use panic_halt as _;

use nucleo_f446re as bsp;


macro_rules! noop { () => (); }

#[entry]
fn main() -> ! {
    loop {
        let mut nucleo: bsp::Nucleo = bsp::Nucleo::<bsp::led::LedDigital>::init().unwrap();
        // Can also use the LED default generic argument with <Nucleo>::init().unwrap();

        loop {
            nucleo.user_led.toggle();
            
            for _ in 0..200_000 {
                noop!();
            }
        }
    }
}
