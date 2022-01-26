use clap::{App, Arg};

pub fn init_cmd() -> App<'static> {
    // Define flags
    let filename = Arg::new("file_name")
        .short('f')
        .long("file_name")
        .takes_value(true)
        .value_name("FILE_NAME")
        .help("Provide a name of the config file");

    // Register command
    return App::new("init")
        .about("Create a default yaml file in the current directory")
        .arg(filename);
}
