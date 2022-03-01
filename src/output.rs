use rand::seq::SliceRandom;
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
                return Ok(());
            }
            Err(err) => {
                spinner.spinner_failure(err.to_string());
                return Err(err);
            }
        };
    }
    pub(crate) fn spinner_start(msg: String) -> Self {
        let spinner = Spinner::new(Spinners::Christmas, msg.to_owned());
        return OutSpinner { spinner, msg };
    }
    pub(crate) fn spinner_success(self, status: String) {
        self.spinner.stop_with_message(
            format!(
                "{}🤙 {}: {}\n",
                color::Fg(color::LightGreen),
                self.msg,
                status
            )
            .into(),
        );
    }
    pub(crate) fn spinner_failure(self, status: String) {
        self.spinner.stop_with_message(
            format!("{}🖕 {}: {}\n", color::Fg(color::Red), self.msg, status).into(),
        )
    }
    pub(crate) fn spinner_close(self) {
        self.spinner
            .stop_with_message(format!("{}🤞 {}\n", color::Fg(color::Cyan), self.msg).into())
    }
}

pub(crate) struct OutSum;

impl OutSum {
    pub(crate) fn sum_success(msg: &str) {
        print!("---\n{}🤘 SUCCESS: {}\n\n", color::Fg(color::Green), msg);
    }
    pub(crate) fn sum_failure(msg: &str) {
        print!("---\n{}👎 FAILURE: {}\n\n", color::Fg(color::Red), msg);
    }
}

pub(crate) struct OutMessage;
const ALIAS: &'static [&'static str] = &["buddy", "mate", "dude", "friend", "dawg", "dear"];

impl OutMessage {
    pub(crate) fn message_empty(msg: &str) {
        print!("{}{}\n", color::Fg(color::LightBlue), msg,);
    }

    pub(crate) fn message_info_with_alias(msg: &str) {
        print!(
            "{}INFO: {}, {}\n",
            color::Fg(color::LightBlue),
            msg,
            ALIAS.choose(&mut rand::thread_rng()).unwrap()
        );
    }
    pub(crate) fn message_info_clean(msg: &str) {
        print!(
            "{}INFO: {}\n",
            color::Fg(color::LightBlue),
            msg,
        );
    }

    pub(crate) fn messageerr(msg: &str) {
        print!("{}ERROR: {}\n", color::Fg(color::LightRed), msg);
    }
}
