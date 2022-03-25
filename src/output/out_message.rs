use console::style;
use rand::seq::SliceRandom;
pub(crate) struct OutMessage;
const ALIAS: &[&str] = &["buddy", "mate", "dude", "friend", "dawg", "dear"];

impl OutMessage {
    pub(crate) fn message_empty(msg: &str) {
        println!(" {}", style(msg).blue());
    }

    pub(crate) fn message_info_with_alias(msg: &str) {
        let prefix = format!("{}", style("[INFO]").blue());
        let msg = format!(
            " {} {}, {}",
            prefix,
            msg,
            ALIAS.choose(&mut rand::thread_rng()).unwrap()
        );
        println!("{}", style(msg).blue());
    }
    pub(crate) fn message_info_clean(msg: &str) {
        let prefix = format!("{}", style("[INFO]"));
        let msg = format!("{} {}", prefix, msg,);
        println!(" {}", style(msg).blue());
    }

    pub(crate) fn message_error(msg: &str) {
        let prefix = format!("{}", style("[ERROR]").red());
        let msg = format!("{} {}", prefix, msg,);
        println!(" {}", style(msg).red());
    }
}
