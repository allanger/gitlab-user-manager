use termion::color;
use rand::seq::SliceRandom;
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
        print!("{}INFO: {}\n", color::Fg(color::LightBlue), msg,);
    }

    pub(crate) fn message_error(msg: &str) {
        print!("{}ERROR: {}\n", color::Fg(color::LightRed), msg);
    }
}
