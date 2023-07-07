use attiny_hal::pac::EEPROM;

static LAST_EEPROM_ADDRESS: u16 = 511;

/// A simple wrapper around EEPROM access for small amounts of data, with wear leveling.
///
/// This operates under the assumption that:
/// - we only need to store 7 bits of data total
/// - we can use the entire EEPROM address space
pub struct Eeprom {
    peripheral: EEPROM,
    current_address: u16,
    pub value: u8,
}

impl Eeprom {
    pub fn new(peripheral: EEPROM) -> Self {
        Eeprom {
            peripheral,
            current_address: 0,
            value: 0,
        }
    }

    pub fn init(&mut self) -> u8 {
        for address in 0..LAST_EEPROM_ADDRESS {
            let value = self.read_at(address);
            // check if the most significant bit is 0
            if value >> 7 == 0 {
                self.value = value;
                self.current_address = address;
                break;
            }
        }

        self.value
    }

    pub fn update(&mut self, value: u8) {
        // Set most significant bit to 0, since it's a flag for finding the current value
        let to_write = value & 127;

        let prev_address = self.current_address;
        if self.current_address >= LAST_EEPROM_ADDRESS {
            self.current_address = 0
        } else {
            self.current_address += 1
        }

        self.erase_at(prev_address);
        self.write_at(self.current_address, to_write);
    }

    fn read_at(&mut self, address: u16) -> u8 {
        self.wait_for_write_complete();
        self.peripheral.eear.write(|w| w.bits(address));
        self.peripheral.eecr.write(|w| w.eere().set_bit());
        self.peripheral.eedr.read().bits()
    }

    fn write_at(&mut self, address: u16, value: u8) {
        self.wait_for_write_complete();
        self.peripheral.eear.write(|w| w.bits(address));
        self.peripheral.eedr.write(|w| w.bits(value));
        self.peripheral
            .eecr
            .write(|w| w.eepm().write().eempe().set_bit());
        self.peripheral.eecr.modify(|_r, w| w.eepe().set_bit());
    }

    fn erase_at(&mut self, address: u16) {
        self.wait_for_write_complete();
        self.peripheral.eear.write(|w| w.bits(address));
        self.peripheral
            .eecr
            .write(|w| w.eepm().erase().eempe().set_bit());
        self.peripheral.eecr.modify(|_r, w| w.eepe().set_bit());
    }

    fn wait_for_write_complete(&mut self) {
        while self.peripheral.eecr.read().eepe().bit_is_set() {}
    }
}
