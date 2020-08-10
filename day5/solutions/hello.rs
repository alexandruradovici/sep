/// Each driver is identified by a unique number
/// numbers higher than 0xa0000000 are unused by standard drivers
pub const DRIVER_NUM: usize = 0xa0000000;

// #[derive(Default)]
pub struct AppData {
    counter: usize,
}

impl Default for AppData {
    fn default() -> AppData {
        AppData { counter: 0 }
    }
}

/// The Hello structure
pub struct Hello {
    grant: kernel::Grant<AppData>,
    callback: kernel::common::cells::OptionalCell<kernel::Callback>,
}

/// The hello implementation
impl Hello {
    pub fn new(grant: kernel::Grant<AppData>) -> Hello {
        Hello {
            grant: grant,
            callback: kernel::common::cells::OptionalCell::empty(),
        }
    }
}

/// The driver system calls implementation
impl kernel::Driver for Hello {
    /// allow will use the default implementation
    fn subscribe(
        &self,
        subscribe_num: usize,
        callback: Option<kernel::Callback>,
        _app_id: kernel::AppId,
    ) -> kernel::ReturnCode {
        match subscribe_num {
            0 => {
                self.callback.insert(callback);
                kernel::ReturnCode::SUCCESS
            }
            _ => kernel::ReturnCode::ENOSUPPORT,
        }
    }
    /// command syscall
    fn command(
        &self,
        command_num: usize,
        _data1: usize,
        _data2: usize,
        app_id: kernel::AppId,
    ) -> kernel::ReturnCode {
        match command_num {
            // command_num 0 is used to verify if the driver exists
            0 => kernel::ReturnCode::SUCCESS,
            1 => {
                kernel::debug!("this is hello driver function 1");
                kernel::ReturnCode::SUCCESS
            }
            2 => self
                .grant
                .enter(app_id, |app, _| {
                    app.counter += 1;
                    kernel::ReturnCode::SuccessWithValue { value: app.counter }
                })
                .unwrap_or_else(|err| err.into()),
            3 => self
                .grant
                .enter(app_id, |app, _| {
                    app.counter += 1;
                    self.callback.map(|callback| callback.schedule(app.counter, 0, 0));
                    kernel::ReturnCode::SuccessWithValue { value: app.counter }
                })
                .unwrap_or_else(|err| err.into()),
            // the command number is not defined
            _ => kernel::ReturnCode::ENOSUPPORT,
        }
    }
}
