use clap::{App, Arg};


pub fn sync_cmd() -> App<'static> {
    let dry_run = Arg::new("dry_run")
        .short('d')
        .takes_value(true)
        .value_name("DRY_RUN")
        .default_value("false")
        .help("Use if you wanna see what's gonna happen without applying new configuration");

    // Register command
    return App::new("sync")
        .about("Sync your config file with GitLab and generate the state file")
        .arg(dry_run);
}
