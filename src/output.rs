use spinners::{Spinner, Spinners};

pub(crate) struct OutSpinner {
    msg: String,
    spinner: Spinner,
}

impl OutSpinner {
    pub(crate) fn spinner_start(msg: String) -> Self {
        let spinner = Spinner::new(Spinners::Weather, msg.to_owned());
        return OutSpinner { spinner, msg };
    }
    pub(crate) fn spinner_success(self, status: String) {
        self.spinner
            .stop_with_message(format!("🤙 {}: {}\n", self.msg, status).into());
    }
    pub(crate) fn spinner_failure(self, status: String) {
        self.spinner
            .stop_with_message(format!("🖕 {}: {}\n", self.msg, status).into())
    }
    pub(crate) fn spinner_close(self) {
        self.spinner
            .stop_with_message(format!("🤞 {}\n", self.msg).into())
    }
}
