use crate::Eeprom;
use attiny_hal::pac::EEPROM;

pub struct Persistence {
    eeprom: Eeprom,
    pub switch1_enabled: bool,
    pub switch2_enabled: bool,
}

impl Persistence {
    pub fn new(peripheral: EEPROM) -> Self {
        let eeprom = Eeprom::new(peripheral);

        Persistence {
            eeprom,
            switch1_enabled: false,
            switch2_enabled: false,
        }
    }

    pub fn init(&mut self) {
        let data = self.eeprom.init();
        (self.switch1_enabled, self.switch2_enabled) = self.parse_value(data);
    }

    pub fn set_switch1_enabled(&mut self, enabled: bool) {
        self.switch1_enabled = enabled;
        self.update();
    }

    pub fn set_switch2_enabled(&mut self, enabled: bool) {
        self.switch2_enabled = enabled;
        self.update();
    }

    fn update(&mut self) {
        let data = if self.switch1_enabled { 1 } else { 0 };
        let mut data = 0;
        if self.switch1_enabled {
            data = data | 1
        }
        if self.switch2_enabled {
            data = data | 2
        }
        self.eeprom.update(data);
    }

    fn parse_value(&self, data: u8) -> (bool, bool) {
        return (data & 1 > 0, data & 2 > 0);
    }
}
