pub(crate) mod init {
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
}

pub(crate) mod search {
    use clap::App;

    fn find_projects() -> App<'static> {
        return App::new("projects").about("Look for GitLab projects");
    }

    fn find_users() -> App<'static> {
        return App::new("users").about("Look for GitLab users");
    }

    fn find_groups() -> App<'static> {
        return App::new("groups").about("Look for GitLab groups");
    }

    pub fn search_cmd() -> App<'static> {
        return App::new("search")
            .about("Create a default yaml file in the current directory")
            .subcommand(find_projects())
            .subcommand(find_users())
            .subcommand(find_groups());
    }
}
