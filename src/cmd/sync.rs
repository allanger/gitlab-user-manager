use crate::{gitlab::GitlabApi, service::v1::SyncService, cmd::Cmd};
use std::io::Result;

use clap::{ArgMatches, Command};

use crate::args::{
    ArgDryRun, ArgFileName, ArgGitlabToken, ArgGitlabUrl, ArgStateDestination, ArgStateSource,
    ArgWriteState, Args,
};

pub(crate) struct SyncCmd {
    dry_run: bool,
    gitlab_token: String,
    gitlab_url: String,
    file_name: String,
    write_state: bool,
    state_destination: String,
    state_source: String,
}

impl<'a> Cmd for SyncCmd {
    type CmdType = SyncCmd;
    fn add() -> Command<'static> {
        Command::new("sync")
            .about("Sync your config file with GitLab and generate the state file")
            .alias("s")
            .after_help("$ gum sync -f gum-config-example.yaml --dry-run")
            .before_help("Use this command if you want to apply changes in your configuration file to GitLab")
            .arg(ArgDryRun::add())
            .arg(ArgGitlabToken::add())
            .arg(ArgGitlabUrl::add())
            .arg(ArgFileName::add())
            .arg(ArgStateDestination::add())
            .arg(ArgStateSource::add())
            .arg(ArgWriteState::add())
    }

    fn prepare(sub_matches: &ArgMatches) -> Result<Self> {
        Ok(SyncCmd {
            dry_run: ArgDryRun::parse(sub_matches)?,
            file_name: ArgFileName::parse(sub_matches)?,
            gitlab_token: ArgGitlabToken::parse(sub_matches)?,
            gitlab_url: ArgGitlabUrl::parse(sub_matches)?,
            state_destination: ArgStateDestination::parse(sub_matches)?,
            state_source: ArgStateSource::parse(sub_matches)?,
            write_state: ArgWriteState::parse(sub_matches)?,
        })
    }
    fn exec(&self) -> Result<()> {
        let mut svc = SyncService::new(
            self.file_name.clone(),
            GitlabApi::new(&self.gitlab_url, &self.gitlab_token)?,
            self.state_source.clone(),
            self.state_destination.clone(),
            self.write_state,
        );
        svc.read_config()?
            .create_states()?
            .compare()?
            .apply(self.dry_run)?
            .update_state()?
            .write_state(self.dry_run)
    }
}

// impl<'a> CmdOld<'a> for SyncCmd {
//     fn exec(&self) -> Result<()> {
//         let gitlab_client = Gitlab::new(&self.gitlab_url, &self.gitlab_token)
//             .map_err(|err| Error::new(ErrorKind::Other, err))?;
//         let mut config_file = match ConfigFile::read(self.file_name.clone()) {
//             Ok(c) => c,
//             Err(err) => return Err(err),
//         };
//         // Read old state
//         let mut old_state: HashMap<u64, AccessUnit> = HashMap::new();

//         if !self.state_source.is_empty() {
//             OutMessage::message_info_with_alias(
//                 format!("I will try to use this file: {}", self.state_source.clone()).as_str(),
//             );
//             old_state = AccessUnit::read_from_file(self.state_source.clone())?
//         } else {
//             if config_file.state.as_str() == "~" || config_file.state.is_empty() {
//                 OutMessage::message_info_with_alias(
//                     "State is not found, I will generate a new one",
//                 );
//             } else {
//                 OutMessage::message_info_with_alias("State is found");
//                 old_state = match serde_json::from_str(config_file.state.as_str()) {
//                     Ok(state) => state,
//                     Err(err) => return Err(Error::new(ErrorKind::InvalidData, err)),
//                 };
//             }
//         }
//         let mut new_state: HashMap<u64, AccessUnit> = HashMap::new();
//         for u in config_file.config.users.iter().clone() {
//             new_state.insert(
//                 u.id,
//                 AccessUnit {
//                     projects: self.configure_projects(u, config_file.config.clone()),
//                     namespaces: self.configure_groups(u, config_file.config.clone()),
//                     entity: EntityType::User,
//                 },
//             );
//         }
//         for u in config_file.config.groups.iter().clone() {
//             new_state.insert(
//                 u.id,
//                 AccessUnit {
//                     projects: self.gr_configure_projects(u),
//                     namespaces: self.gr_configure_groups(u),
//                     entity: EntityType::Group,
//                 },
//             );
//         }

//         let actions = self.compare_states(old_state.clone(), new_state);
//         let state: String = match self.apply(actions, &gitlab_client, &mut old_state, self.dry_run)
//         {
//             Ok(_) => serde_json::to_string(&old_state).unwrap(),
//             Err(err) => {
//                 OutMessage::message_error(
//                     format!(
//                         "This error happened while I was applying:\n {}\nI'll save the intermediate state",
//                         err
//                     )
//                     .as_str(),
//                 );
//                 serde_json::to_string(&old_state).unwrap()
//             }
//         };
//         if !self.dry_run {
//             config_file.state = state;
//         }

//         if self.write_state {
//             match AccessUnit::write_to_file(old_state, self.state_destination.clone()) {
//                 Ok(_) => {
//                     let msg = format!(
//                         "State is saved, check it out\n $ cat {}",
//                         self.state_destination.clone()
//                     );
//                     OutMessage::message_empty(msg.as_str());
//                 }
//                 Err(_) => OutMessage::message_empty("Couldn't save state to file"),
//             };
//         }

//         match config_file.write(self.file_name.clone()) {
//             Ok(_) => Ok(()),
//             Err(err) => Err(err),
//         }
//     }
// }
