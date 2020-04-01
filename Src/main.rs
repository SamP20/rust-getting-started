#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate panic_halt;

use rt::entry;
use stm32f4::stm32f429;


#[entry]
fn main() -> ! {
    let peripherals = stm32f429::Peripherals::take().unwrap();

    let rcc = &peripherals.RCC;
    rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());

    let gpiob = &peripherals.GPIOB;

    gpiob.moder.modify(|_, w| w.moder0().output());
    gpiob.bsrr.write(|w| w.bs0().set_bit());

    loop {
        continue;
    }
}