use clap::ArgMatches;

pub fn teams_pkg(sub_matches: &ArgMatches) -> () {
    println!("FUUUUCKING NOT COOOL YET");
    match sub_matches.subcommand() {
        Some(("create", sub_matches)) => {
            println!("FUUUUCKING COOOL");
        }
        _ => unreachable!(),
    }
}
