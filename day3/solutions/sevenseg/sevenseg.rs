use core::cell::Cell;
use kernel::hil::gpio::Pin;
use kernel::{AppId, Driver, ReturnCode};

pub const DRIVER_NUM: usize = 0xa0000001;

const SEGMENT_MAP: [u8; 10] = [0xC0, 0xF9, 0xA4, 0xB0, 0x99, 0x92, 0x82, 0xF8, 0x80, 0x90];
const SEGMENT_SELECT: [u8; 4] = [0xF1, 0xF2, 0xF4, 0xF8];

pub struct SevenSeg {
    latch_pin: &'static dyn Pin,
    clock_pin: &'static dyn Pin,
    data_pin: &'static dyn Pin,
    number: Cell<u8>,
    position: Cell<u8>,
    enabled: Cell<bool>,
}

impl SevenSeg {
    pub fn new(
        latch_pin: &'static dyn Pin,
        clock_pin: &'static dyn Pin,
        data_pin: &'static dyn Pin,
    ) -> SevenSeg {
        latch_pin.make_output();
        clock_pin.make_output();
        data_pin.make_output();
        SevenSeg {
            latch_pin,
            clock_pin,
            data_pin,
            number: Cell::new(0),
            position: Cell::new(0),
            enabled: Cell::new(false),
        }
    }

    pub fn shift_out(&self, value: u8) {
        for i in (0..=7).rev() {
            if ((value >> i) & 0x01) == 1 {
                self.data_pin.set();
            } else {
                self.data_pin.clear();
            }
            self.clock_pin.set();
            self.clock_pin.clear();
        }
    }

    pub fn write_number_to_position(&self, number: u8, position: u8) {
        self.shift_out(number);
        self.shift_out(position);
        self.latch_pin.set();
        self.latch_pin.clear();
    }
}

impl Driver for SevenSeg {
    fn command(
        &self,
        command_num: usize,
        data1: usize,
        data2: usize,
        _app_id: AppId,
    ) -> ReturnCode {
        match command_num {
            0 => ReturnCode::SUCCESS,
            1 => {
                self.enabled.set(true);
                self.write_number_to_position(
                    SEGMENT_MAP[self.number.get() as usize],
                    SEGMENT_SELECT[self.position.get() as usize],
                );
                ReturnCode::SUCCESS
            }
            2 => {
                self.write_number_to_position(0xFF, SEGMENT_SELECT[self.position.get() as usize]);
                self.enabled.set(false);
                ReturnCode::SUCCESS
            }
            3 => {
                self.number.set(data1 as u8);
                self.position.set(data2 as u8);

                if self.enabled.get () {
                    self.write_number_to_position(SEGMENT_MAP[data1], SEGMENT_SELECT[data2]);
                }
                ReturnCode::SUCCESS
            }
            4 => ReturnCode::SuccessWithValue {
                value: self.number.get() as usize,
            },
            5 => ReturnCode::SuccessWithValue {
                value: self.position.get() as usize,
            },
            _ => ReturnCode::ENOSUPPORT,
        }
    }
}
