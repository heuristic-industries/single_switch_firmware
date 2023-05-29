use crate::SwitchTimer;
use attiny_hal::{clock, delay};
use core::fmt::Debug;
use embedded_hal::{
    digital::v2::{InputPin, OutputPin},
    prelude::_embedded_hal_blocking_delay_DelayMs,
};

pub struct ToggleSwitch<Input, Output> {
    input: Input,
    output: Output,
    pub active: bool,
    previous_state: bool,
    delay: delay::Delay<clock::MHz1>,
}

impl<Input, Output> ToggleSwitch<Input, Output>
where
    Input: InputPin,
    Output: OutputPin,
    Input::Error: Debug,
    Output::Error: Debug,
{
    pub fn new(input: Input, output: Output, active: bool) -> Self {
        let delay = delay::Delay::<clock::MHz1>::new();

        ToggleSwitch {
            input,
            output,
            active,
            previous_state: false,
            delay,
        }
    }

    pub fn init(&mut self) {
        if self.active {
            self.pulse();
        }
    }

    pub fn on_change(&mut self, timer: &mut SwitchTimer) -> bool {
        let pressed = self.is_pressed();

        if pressed == self.previous_state || !timer.debounce.threshold_reached {
            return false;
        }

        timer.debounce.reset();

        self.previous_state = pressed;

        if pressed {
            timer.hold.reset();

            self.set_state(!self.active);
        } else if timer.hold.threshold_reached {
            self.set_state(false);
        }

        true
    }

    fn is_pressed(&mut self) -> bool {
        self.input.is_low().unwrap()
    }

    fn set_state(&mut self, state: bool) {
        self.active = state;
        self.pulse();
    }

    fn pulse(&mut self) {
        self.output.set_low().unwrap();
        self.delay.delay_ms(100 as u8);
        self.output.set_high().unwrap();
    }
}
