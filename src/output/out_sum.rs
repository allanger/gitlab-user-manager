use termion::color;
pub(crate) struct OutSum;

impl OutSum {
    pub(crate) fn sum_success(msg: &str) {
        print!("---\n{}🤘 SUCCESS: {}\n\n", color::Fg(color::Green), msg);
    }
    pub(crate) fn sum_failure(msg: &str) {
        print!("---\n{}👎 FAILURE: {}\n\n", color::Fg(color::Red), msg);
    }
}
