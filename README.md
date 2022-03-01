# GUM
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://GitHub.com/Naereen/StrapDown.js/graphs/commit-activity)
![Maintainer](https://img.shields.io/badge/maintainer-allanger-blue)

## Usage 
### Download 
- Get executable from github releases

Prebuilt binaries exist for **Linux x86_64** and **MacOS arm64** and **x86_64**
```
$ export VERSION=v0.0.2 && curl -LJO  https://github.com/allanger/gitlab-user-manager/releases/download/$VERSION/gum-$VERSION-$(arch)-$(uname) && mv gum-$VERSION-$(arch)-$(uname) gum && chmod +x gum
$ gum -h
```
### Docker

You can use the `latest` or a `tagged` docker image
```
$ docker pull ghcr.io/allanger/gitlab-user-manager:latest
$ docker run ghcr.io/allanger/gitlab-user-manager:latest gum -h
$ docker run ghcr.io/allanger/gitlab-user-manager:latest $PWD:/config gum init
```

**Note:** Images with versions 0.0.1 and 0.0.2 are broken, because the workdir is set to `/bin`

### Build from source
1. Build binary
```
$ cargo build --release
``` 
2. Run `gum help`

3. Init new config
```
$ gum init
```
It will create an empty config file

4. Work with teams

`teams` commands takes team names as arguments, so if you want to modify not default team pass an argumanet with a team name
```
$ gum teams help
$ gum teams add-project -u PROJECT_ID # will add a project to the default team
$ gum teams add-project -u PROJECT_ID -n backend-team # will add project to the backend-team
```

5. There is a command to search for project IDs (Search currently lists only internal projects for specified token)
```
$ gum projects search PROJECT_NAME
```

6. Work with users
```
$ gum users help
$ gum users create USER_ID
```

7. Add projects to users
```
$ gum users add-project $USER_ID -i $PROJECT_ID -a $ACCESS_LEVEL
```

7. Add projects to teams
```
$ gum teams add-project -i $PROJECT_ID -a $ACCESS_LEVEL $TEAM_NAME
```

8. Add user to team 
```
$ gum users add-team USER_ID team-1 team-2
```
9. Apply config
```
## to see what's gonna happen
$ gum sync --dry-run true 
## to apply 
$ gum sync 
```
`sync` command will compare current state (if exists) and apply changes. Then the new state will be saved as gum-state.yaml. 