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
- [ ] Write good documentation in README.md

# Nice to have
- [ ] Add a `describe` command foreach config entity 
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
- [ ] Add a `refresh` command to update gitlab Projects/Groups/Users name
- [ ] Get rid of Ownerships and migrate to Groups
- [ ] Add a head group to the config. Gum should be able to manage access only in these groups. (Maybe use a better name for this)
  ```
  meta: 
    head_groups: 
      - group_1
      - group_2
  ```
- [ ] Add an ability to remove user completely from the groups specified in the head_groups. Maybe add a flag the `sync` command like that:
  ```
  $ gum sync --cleanup
  ```
And remove each user, which is being updated anyhow, from groups provided via head_groups (remove from groups, sub groups and projects)

- [ ] Remove extra fields from state. State should only contain IDs and access_level
- [ ] Auto check for updates
- [ ] Add feature for inviting users, if it's possible. 
  When user is invited, he won't have an id until he confirms invitation. But after he confirms, he will be added to projects where he's been invited. So I think it should look like that:
  -  New object is added to config
  ```
  invites: 
   id: invite_id
   projects: [] 
   teams: []
   groups: []
  ```
  - On each sync this invites should be checked for confirmation (if there so no such possibility, the whole invite ting seems impossible)
  - If it's confirmed, the user should be automatically added to users and to state
- [ ] Add patterns for checking users emails and usernames
# Maybe nice to have
- [ ] Generate HTML from the current state to publish it via gitlab pages
- [ ] Add patterns for checking projects and groups names