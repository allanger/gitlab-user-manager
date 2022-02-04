pub(crate) mod srv {
    use clap::ArgMatches;
    use std::io::Error;

    struct Srv<'a> {
        sub_matches: &'a ArgMatches,
    }

    trait SrvActions {
        fn init(&self) -> Option<Error>;
    }

    impl<'a> SrvActions for Srv<'a> {
        fn init(&self) -> Option<Error> {
          init_mod::init()
            // None
        }
    }

    mod init_mod {
        use std::io::Error;

        pub fn init() -> Option<Error> {
            None
        }
    }
}
