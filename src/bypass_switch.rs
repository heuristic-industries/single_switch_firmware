use crate::{Persistence, ToggleSwitch, SwitchTimer};
use attiny_hal::port::{
    mode::{Input, Output, PullUp},
    Pin, PB3, PB4,
};

pub struct BypassSwitch {
    switch: ToggleSwitch<Pin<Input<PullUp>, PB3>, Pin<Output, PB4>>,
}

impl BypassSwitch {
    pub fn new(pins: attiny_hal::port::Pins, active: bool) -> Self {
        let input = pins.pb3.into_pull_up_input();
        let output = pins.pb4.into_output();
        let mut switch = ToggleSwitch::new(input, output, active);
        switch.init();

        BypassSwitch { switch }
    }

    pub fn on_change(&mut self, timer: &mut SwitchTimer, persistence: &mut Persistence) {
        let did_change = self.switch.on_change(timer);
        if did_change {
            persistence.set_bypass(self.switch.active);
        }
    }
}
