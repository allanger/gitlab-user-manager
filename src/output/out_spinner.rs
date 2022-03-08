use spinners::{Spinner, Spinners};
use std::io::Result;
use termion::color;

pub(crate) struct OutSpinner {
    msg: String,
    spinner: Spinner,
}

impl OutSpinner {
    // I wanted to wrap a function with a spinner but not user how to implement it yet
    pub(crate) fn spinner_wrapper<F>(f: F, msg: String) -> Result<()>
    where
        // The closure takes no input and returns nothing.
        F: FnOnce() -> Result<String>,
    {
        let spinner = OutSpinner::spinner_start(msg);
        match f() {
            Ok(msg) => {
                spinner.spinner_success(msg);
                Ok(())
            }
            Err(err) => {
                spinner.spinner_failure(err.to_string());
                Err(err)
            }
        }
    }
    pub(crate) fn spinner_start(msg: String) -> Self {
        let spinner = Spinner::new(Spinners::Christmas, msg.to_owned());
        OutSpinner { spinner, msg }
    }
    pub(crate) fn spinner_success(self, status: String) {
        self.spinner.stop_with_message(format!(
            "{}🤙 {}: {}\n",
            color::Fg(color::LightGreen),
            self.msg,
            status
        ));
    }
    pub(crate) fn spinner_failure(self, status: String) {
        self.spinner.stop_with_message(format!(
            "{}🖕 {}: {}\n",
            color::Fg(color::Red),
            self.msg,
            status
        ))
    }
    pub(crate) fn spinner_close(self) {
        self.spinner
            .stop_with_message(format!("{}🤞 {}\n", color::Fg(color::Cyan), self.msg))
    }
}
