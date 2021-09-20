#![no_std]
#![no_main]

extern crate panic_halt;

use arduino_uno::hal::port::mode::Output;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::prelude::*;

fn stutter_blink(led: &mut PB5<Output>, times: usize) {
	(0..times).for_each(|_| {
		let _ = led.set_high();
		arduino_uno::delay_ms(10);
		let _ = led.set_low();
		arduino_uno::delay_ms(100);
	});
}

#[arduino_uno::entry]
fn main() -> ! {
	let peripherals = arduino_uno::Peripherals::take().unwrap();

	let mut pins = arduino_uno::Pins::new(peripherals.PORTB, peripherals.PORTC, peripherals.PORTD);

	let mut led = pins.d13.into_output(&mut pins.ddr);

	loop {
		stutter_blink(&mut led, 12);
	}
}
