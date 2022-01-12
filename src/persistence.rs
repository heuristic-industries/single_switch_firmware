use crate::Eeprom;
use attiny_hal::pac::EEPROM;

pub struct Persistence {
    eeprom: Eeprom,
    pub bypass_enabled: bool,
}

impl Persistence {
    pub fn new(peripheral: EEPROM) -> Self {
        let eeprom = Eeprom::new(peripheral);

        Persistence {
            eeprom,
            bypass_enabled: false,
        }
    }

    pub fn init(&mut self) {
        let data = self.eeprom.init();
        self.bypass_enabled = self.parse_value(data);
    }

    pub fn set_bypass(&mut self, enabled: bool) {
        self.bypass_enabled = enabled;
        self.update();
    }

    fn update(&mut self) {
        let data = if self.bypass_enabled { 1 } else { 0 };
        self.eeprom.update(data);
    }

    fn parse_value(&self, data: u8) -> bool {
        data > 0
    }
}
