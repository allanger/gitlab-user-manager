# MVP
What needs to be done before the tool can actually be used.
- [ ] Implement all required gitlab API
  - APIs
    - Add members
    - Delete members
    - Update members
  - On adding handle error User Already Exists
  - On removing handle error User Not Found 
  - Add corresponding output to error handling (Already Added, Not Found)

- [ ] Get rid of billions of loops and start using HashMaps
- [ ] Add tests
- [ ] Create a Dockerfile and GitHub Actions for tests and releases
- [ ] Refactor whole arguments module. Maybe rewrite it to methods or something
- [ ] Refactor error handling

# Nice to have
- [ ] Add describe command foreach config entity 
  ```
  $ gum describe user allanger
  username: $GITLAB_USER_NAME
  id: $GITLAB_USER_ID
  projects: $LIST_OF_PROJECTS 
  teams: $LIST_OF_TEAMS
  ownerships: $LIST_OF_OWNERSHIPS
  
  $ gum describe team team_1
  name: team_1
  projects: $LIST_OF_PROJECTS 
  ```

- [ ] Async actions
- [ ] Get rid of state file
- [ ] Use spinners for output (EG https://github.com/console-rs/indicatif)
- [ ] Add auto suggestions for every command
- [ ] Start versioning the config file and add possibility to migrate from to a newer version. And add kinda annotation to let gum know it's a gum-config. Something like that
  ```
  meta: 
    kind: gum-config
    version: v1beta
  teams: []
  users: []
  ```
- [ ] Add ability to set custom file names
- [ ] Add information about the latest sync (When, Who)
- [ ] Add possibility to write sync logs to file to have a better history
- [ ] Rollback on errors or save the actual state if rollback is failed too.
- [ ] Put temp state (dry-run) to the /tmp/gum folder and remove old ones
