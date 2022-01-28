use std::io::{Seek, SeekFrom};

use clap::ArgMatches;

use crate::types::types;

pub fn teams_pkg(sub_matches: &ArgMatches) -> () {
    match sub_matches.subcommand() {
        Some(("create", sub_matches)) => create(sub_matches.value_of("TEAM_NAME").expect("Team name is required")),
        _ => unreachable!(),
    }
}

fn create(team_name: &str) -> () {
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open("gum-config.yaml")
        .unwrap();
    let mut d: types::Config = serde_yaml::from_reader(&f).expect("required");
    f.seek(SeekFrom::Start(0)).expect("msg");

    let new_team = types::Team {
        name: team_name.to_string(),
        projects: None,
    };
    
    d.teams.as_mut().unwrap().extend([new_team]);
    println!("Read YAML string: {:?}", d);
    serde_yaml::to_writer(&f, &d).unwrap();
}
