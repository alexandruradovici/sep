use core::cell::Cell;
use kernel::hil::gpio::Pin;
use kernel::hil::time::{Alarm, AlarmClient, Frequency};
use kernel::{AppId, Driver, ReturnCode, debug};

/// Each driver is identified by a unique number
pub const DRIVER_NUM: usize = 0xb0000000;
const DIGIT_MAP: [u8; 11] = [
    0xC0, 0xF9, 0xA4, 0xB0, 0x99, 0x92, 0x82, 0xF8, 0x80, 0x90, 0xFF,
];
const SEGMENT_MAP: [u8; 4] = [0xF1, 0xF2, 0xF4, 0xF8];

pub struct SevenSeg<A: 'static + Alarm<'static>> {
    latch_pin: &'static dyn Pin,
    data_pin: &'static dyn Pin,
    clock_pin: &'static dyn Pin,

    number: Cell<Option<u16>>,
    digits: Cell<[u8; 4]>,

    alarm: &'static A,
    //brightness: f32, // TODO: implement brightness changing (possible with alarm. higher refresh rate = smaller delay = brighter)
}

/// The hello implementation
impl<A: 'static + Alarm<'static>> SevenSeg<A> {
    pub fn new(
        latch_pin: &'static dyn Pin,
        data_pin: &'static dyn Pin,
        clock_pin: &'static dyn Pin,

        alarm: &'static A,
    ) -> SevenSeg<A> {
        latch_pin.make_output();
        data_pin.make_output();
        clock_pin.make_output();
        SevenSeg {
            latch_pin: latch_pin,
            data_pin: data_pin,
            clock_pin: clock_pin,

            number: Cell::new(None),
            digits: Cell::new([DIGIT_MAP[10], DIGIT_MAP[10], DIGIT_MAP[10], DIGIT_MAP[10]]),

            alarm: alarm,
        }
    }

    fn turn_display_off(&self) -> () {
        self.latch_pin.clear();
        self.shift_out(0xFF);
        self.shift_out(SEGMENT_MAP[0]);
        self.latch_pin.set();
    }

    fn turn_display_on(&self) -> () {
        match self.number.get() {
            None => kernel::debug!("No number has been set"),
            Some(x) => {
                let digit0 = if x / 1000 > 0 { (x % 10000) / 1000 } else { 10 };
                let digit1 = if x / 100 > 0 { (x % 1000) / 100 } else { 10 };
                let digit2 = if x / 10 > 0 { (x % 100) / 10 } else { 10 };
                let digit3 = x % 10;

                self.digits.set([
                    DIGIT_MAP[digit0 as usize],
                    DIGIT_MAP[digit1 as usize],
                    DIGIT_MAP[digit2 as usize],
                    DIGIT_MAP[digit3 as usize],
                ])
            }
        }
    }

    fn display_digits(&self) -> () {
        for i in 0..4 {
            self.latch_pin.clear();
            self.shift_out(self.digits.get()[i]);
            self.shift_out(SEGMENT_MAP[i]);
            self.latch_pin.set();
        }
    }

    fn shift_out(&self, value: u8) -> () {
        for i in (0..8).rev() {
            if ((value >> i) & 0x01) == 1 {
                self.data_pin.set();
            } else {
                self.data_pin.clear();
            }
            self.clock_pin.set();
            self.clock_pin.clear();
        }
    }
}

/// The driver system calls implementation
impl<A: 'static + Alarm<'static>> Driver for SevenSeg<A> {
    /// subscribe and allow will use the default implementation
    /// command syscall
    fn command(
        &self,
        command_num: usize,
        data1: usize,
        _data2: usize,
        _app_id: AppId,
    ) -> ReturnCode {
        match command_num {
            // command_num 0 is used to verify if the driver exists
            0 => ReturnCode::SUCCESS,
            1 => {
                self.turn_display_on();
                debug! ("setting alarm");
                debug! ("{:?}", <A::Frequency>::frequency());
                self.alarm.set_alarm(
                    self.alarm
                        .now()
                        .wrapping_add(<A::Frequency>::frequency() / 8000),
                );
                self.alarm.enable();
                ReturnCode::SUCCESS
            }
            2 => {
                self.alarm.disable();
                self.turn_display_off();
                ReturnCode::SUCCESS
            },
            3 => {
                self.number.set(Some(data1 as u16));
                debug! ("set number to {}", self.number.get().unwrap());
                ReturnCode::SUCCESS
            }
            // the command number is not defined
            _ => ReturnCode::ENOSUPPORT,
        }
    }
}

impl<A: 'static + Alarm<'static>> AlarmClient for SevenSeg<A> {
    fn fired(&self) {
        self.display_digits();
        // next alarm
        self.alarm.set_alarm(
            self.alarm
                .now()
                .wrapping_add(<A::Frequency>::frequency() / 8000)); // 8000 is the maximum and therefore the brightest
    }
}