use console::{style, Emoji};
use indicatif::{ProgressBar, ProgressStyle};
use std::{io::Result, time::Duration};

pub(crate) struct OutSpinner {
    msg: String,
    spinner: ProgressBar,
}
static CLOCK_1: Emoji<'_, '_> = Emoji("🕛", "|");
static CLOCK_2: Emoji<'_, '_> = Emoji("🕝", "/");
static CLOCK_3: Emoji<'_, '_> = Emoji("🕓", "-");
static CLOCK_4: Emoji<'_, '_> = Emoji("🕟", "\\");
static CLOCK_5: Emoji<'_, '_> = Emoji("🕡", "|");
static CLOCK_6: Emoji<'_, '_> = Emoji("🕗", "/");
static CLOCK_7: Emoji<'_, '_> = Emoji("🕘", "-");
static SUCCESS: Emoji<'_, '_> = Emoji("🤙", "S");
static FAILURE: Emoji<'_, '_> = Emoji("🖕", "F");
static FINISH: Emoji<'_, '_> = Emoji("🤞", "?");

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
        let chars = format!("{CLOCK_1}{CLOCK_2}{CLOCK_3}{CLOCK_4}{CLOCK_5}{CLOCK_6}{CLOCK_7}");

        let spinner_style = ProgressStyle::default_spinner()
            .tick_chars(&chars)
            .template(" [ {spinner} ] {msg}: ... ");

        let spinner = ProgressBar::new_spinner()
            .with_style(spinner_style)
            .with_message(msg.clone());

        spinner.enable_steady_tick(Duration::from_secs(2));
        OutSpinner { msg, spinner }
    }
    pub(crate) fn spinner_success(self, status: String) {
        self.spinner.finish_and_clear();
        println!(" [ {} ] {}: {}", SUCCESS, self.msg, style(status).green());
    }
    pub(crate) fn spinner_failure(self, status: String) {
        self.spinner.finish_and_clear();
        println!("[ {} ] {}: {}", FAILURE, self.msg, style(status).green());
    }
    pub(crate) fn spinner_close(self) {
        self.spinner.finish_and_clear();
        println!("[ {} ] {}", FINISH, self.msg);
    }
}
