#![feature(llvm_asm)]
#![no_main]
#![no_std]
#![feature(abi_avr_interrupt)]

use panic_halt as _;

use avr_device::interrupt;
use avr_device::interrupt::{free, Mutex};

use cell::RefCell;
use core::cell;

mod toggle_switch;
use toggle_switch::ToggleSwitch;

mod bypass_switch;
use bypass_switch::BypassSwitch;

mod timer;
use timer::Timer;

mod switch_timer;
use switch_timer::SwitchTimer;

mod eeprom;
use eeprom::Eeprom;

mod persistence;
use persistence::Persistence;

type InterruptFlag = Mutex<RefCell<bool>>;
static TIMER_INTERRUPT: InterruptFlag = Mutex::new(RefCell::new(false));
static BUTTON_INTERRUPT: InterruptFlag = Mutex::new(RefCell::new(false));

#[attiny_hal::entry]
fn main() -> ! {
    let peripherals = attiny_hal::pac::Peripherals::take().unwrap();
    let pins = attiny_hal::port::Pins::new(peripherals.PORTB);

    let mut persistence = Persistence::new(peripherals.EEPROM);
    persistence.init();

    // Configure timer/counter 0 to count up and fire the TIMER0_COMPA
    // at a regular interval to act as a clock for our timers
    // The compare interrupt is set to fire roughly every 1ms:
    // 1 / (1Mhz / 8) * 125 = 1ms
    let tc0 = peripherals.TC0;
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.tccr0b.write(|w| w.cs0().prescale_8());
    tc0.ocr0a.write(|w| unsafe { w.bits(124_u8) });
    tc0.timsk.write(|w| w.ocie0a().bit(true));

    // Enable pin change interrupt for PB3 to detect switch changes
    peripherals.EXINT.gimsk.write(|w| w.pcie().set_bit());
    peripherals
        .EXINT
        .pcmsk
        .write(|w| unsafe { w.bits(0b00001000) });

    let mut bypass_timer = SwitchTimer::new();
    let mut bypass = BypassSwitch::new(pins, persistence.bypass_enabled);

    unsafe { avr_device::interrupt::enable() };

    loop {
        avr_device::asm::sleep();

        let (timer, button) = free(|cs| {
            (
                TIMER_INTERRUPT.borrow(cs).replace(false),
                BUTTON_INTERRUPT.borrow(cs).replace(false),
            )
        });

        if timer {
            bypass_timer.tick();
        }
        if button {
            bypass.on_change(&mut bypass_timer, &mut persistence);
        }
    }
}

#[interrupt(attiny85)]
fn TIMER0_COMPA() {
    free(|cs| {
        TIMER_INTERRUPT.borrow(cs).replace(true);
    })
}

#[interrupt(attiny85)]
fn PCINT0() {
    free(|cs| {
        BUTTON_INTERRUPT.borrow(cs).replace(true);
    })
}
