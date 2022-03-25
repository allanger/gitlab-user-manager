use console::{style, Emoji};
use termion::color;
pub(crate) struct OutExtra;

static SUCCESS: Emoji<'_, '_> = Emoji("🤘", "S");
static FAILURE: Emoji<'_, '_> = Emoji("🖕", "F");

impl OutExtra {
    pub(crate) fn welcome_message(msg: &str, news: &'static [&'static str]) {
        OutExtra::empty_line();
        println!(" {}", style(msg).black());
        OutExtra::empty_line();
        if news.len() > 0 {
            println!(
                " {}",
                style("I've got news for you, dude")
                    .black()
                    .underlined()
                    .bold()
            );
            for n in news.iter() {
                println!(" * {}", style(n).cyan());
            }
        }
        OutExtra::empty_line();
    }

    pub(crate) fn empty_line() {
        println!("")
    }

    pub(crate) fn sum_success(msg: &str) {
        OutExtra::empty_line();
        println!(" {} SUCCESS: {}", SUCCESS, style(msg).green());
    }
    pub(crate) fn sum_failure(msg: &str) {
        OutExtra::empty_line();
        println!(" {} FAILURE: {}", FAILURE, style(msg).red());
    }
}
