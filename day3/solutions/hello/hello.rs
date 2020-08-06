use kernel::{Driver, AppId, ReturnCode};
use core::cell::Cell;
use kernel::debug;
/// Each driver is identified by a unique number
///
/// numbers higher than 0xa0000000 are unused by standard drivers
pub const DRIVER_NUM: usize = 0xa0000000;
/// The Hello structure
pub struct Hello {
    nr: Cell<usize>
}
/// The hello implementation
impl Hello {
    pub fn new() -> Hello {
        Hello {
            nr: Cell::new (0)
        }
    }
}
/// The driver system calls implementation
impl Driver for Hello {
    /// subscribe and allow will use the default implementation
    /// command syscall
    fn command(&self, command_num: usize, _data1: usize, _data2: usize, _app_id: AppId) -> ReturnCode {
        match command_num {
            // command_num 0 is used to verify if the driver exists
            0 => ReturnCode::SUCCESS,
            1 => { debug! ("Print Hello"); ReturnCode::SUCCESS }
            2 => {
                // modify number
                self.nr.set (self.nr.get () + 1);
                ReturnCode::SuccessWithValue { value: self.nr.get() }
            }
            // the command number is not defined
            _ => ReturnCode::ENOSUPPORT,
        }
    }
}