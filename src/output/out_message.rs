use rand::seq::SliceRandom;
use termion::color;
pub(crate) struct OutMessage;
const ALIAS: &[&str] = &["buddy", "mate", "dude", "friend", "dawg", "dear"];

impl OutMessage {
    pub(crate) fn message_empty(msg: &str) {
        println!("{}{}", color::Fg(color::LightBlue), msg,);
    }

    pub(crate) fn message_info_with_alias(msg: &str) {
        println!(
            "{}INFO: {}, {}",
            color::Fg(color::LightBlue),
            msg,
            ALIAS.choose(&mut rand::thread_rng()).unwrap()
        );
    }
    pub(crate) fn message_info_clean(msg: &str) {
        println!("{}INFO: {}", color::Fg(color::LightBlue), msg,);
    }

    pub(crate) fn message_error(msg: &str) {
        println!("{}ERROR: {}", color::Fg(color::LightRed), msg);
    }

    pub(crate) fn message_important(msg: &str) {
        println!("{}IMPORTANT: {}", color::Fg(color::Red), msg);
    }

}
