mod args;
mod cli;
mod cmd;
mod gitlab;
mod output;
mod service;
mod store;
mod types;
use output::out_extra::OutExtra;
use std::process::exit;

const MESSAGE_OF_THE_DAY: &str = "☮️  Fight war, not wars ☮️";
const NEWS: &[&str] = &[
    "IMPORTANT: It's not a dead project yet, but I'm just not using gitlab anymore, so development is kinda slow"
];

fn main() {
    OutExtra::welcome_message(MESSAGE_OF_THE_DAY, NEWS);

    match cli::exec(cli::build().get_matches()) {
        Err(err) => {
            OutExtra::sum_failure(&err.to_string());
            exit(1);
        }
        Ok(_) => {
            OutExtra::sum_success("It was fun, wasn't it?");
        }
    }
}
