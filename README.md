# GUM
> or just Gitlab User Manager
> 

[![Version build](https://github.com/allanger/gitlab-user-manager/actions/workflows/build-version.yaml/badge.svg)](https://github.com/allanger/gitlab-user-manager/actions/workflows/build-version.yaml)
[![Version container](https://github.com/allanger/gitlab-user-manager/actions/workflows/container-version.yaml/badge.svg)](https://github.com/allanger/gitlab-user-manager/actions/workflows/container-version.yaml)
[![Stable container](https://github.com/allanger/gitlab-user-manager/actions/workflows/container-stable.yaml/badge.svg)](https://github.com/allanger/gitlab-user-manager/actions/workflows/container-stable.yaml)

## What's this?

As I said, it's just a gitlab user manager. The better question, why do you want to use it? **Because gitlab user management is such a mess**!

*What? You don't think so? Well...I will try to explain my point after I describe what this project exists for.*

In short, using **gum** you can control who and how can access your projects and groups with the everything-as-code approach. Just create a config file and sync it. 

---

**Back to GitLab.**

Let's imagine you have two big projects in your company **IT-Leaders**: 
- The Best Crypto Exchanger
- Just a Sock Shop

Your Gitlab projects structure is probably looking like this:

- **Group**: IT-Leaders
  - **SubGroup**: Common
    - **Project**: IT-Leaders Go Lib
    - **Project**: IT-Leaders Frontend Components
  - **SubGroup**: The Best Crypto Exchanger
    - **Project**: Crypto Exchanger Web
    - **Project**: Crypto Exchanger Api
    - **Project**: Crypto Exchanger Micro Service
    - **SubGroup**: Crypto Exchanger Devops:
      - **Project**: Crypto Exchanger Infrastructure
  - **SubGroup**: Just a Sock Shop
    - **Project**: Sock Shop Web
    - **Project**: Sock Shop Api
    - **SubGroup**: Crypto Exchanger Devops:
      - **Project**: Crypto Exchanger Scripts
    - **Project**: Crypto Exchanger Documentation

Already tough, huh? 

And now you have to give users access to you groups and projects. I see several ways of doing that:
1. You can just add them to the main parent group
2. You can add them directly to projects
3. You can add them to subgroups
4. You can create one more subgroups with another subgroups level, add those subgroups to projects and add users just there. 

Now it a time for pros and cons...
- Way 1
  - Pros: Easy as heck
  - Cons: Everything else. At least should backend and frontend developers have similar access to backend and frontend projects? Or junior devs and tech leads? You can't really control access with this approach.
- Way 2
  - Pros: Very flexible.
  - Cons: As for me this is the right approach. But when you have many projects, this is gonna be incredible to add users and then to remove them. The best option here is to write a script or something. (but don't do that, because **gum** already's here for ya)
- Way 3 
  - Pros: More control that with the **way 1** and it's still pretty easy
  - Cons: But with this approach you will have to split users by projects that are working on. For example, should **Just a Sock Shop** backend developer have and access to the **Crypto Exchanger Micro Service**? If yes, what will you do? Just add him to the project or to the whole subgroup? If you add him to the whole subgroup, he will have a developer access to **Crypto Exchanger Web**, but should he? That way doesn't seem to work for me.
- Way 4
  - Pros: This is a good one. Flexible, good control, easy to add and to remove.
  - Cons: You will have to create a huge amount of user management subgroups to set it up. And after you finally have done it, you will realize that users with the maintainer access are adding another users to the projects directly. And the whole system is not working anymore, because the mess is already in you Gitlab 

Oh, wait! There is another way. Just use **gum**. It's easy as a pie. I will show you how.

## Install 
### Download 

Get executable from github releases

Prebuilt binaries exist for **Linux x86_64** and **MacOS arm64** and **x86_64**

Don't forget to add the binary to $PATH
```
$ curl https://raw.githubusercontent.com/allanger/gitlab-user-manager/main/scripts/download_gum.sh | bash
$ gum -h
```
### Docker

You can use the `latest` or a `tagged` docker image
```
$ docker pull ghcr.io/allanger/gitlab-user-manager:latest
$ docker run ghcr.io/allanger/gitlab-user-manager:latest gum -h
$ docker run ghcr.io/allanger/gitlab-user-manager:latest $PWD:/config gum init
```

### Build from source
1. Build binary
```
$ cargo build --release
``` 
2. Run `gum help`

### Auto completions

`Gum` can generate completions (but I've tested them only with ZSH)

To generate them, use the `generate` command
```
$ gum generate --help
$ gum generate -s zsh # if you don't provide the -s argument, the default shell will be taken, which is ZSH
```

Gum will create a `_gum` file in the current dir, which you should put to your $FPATH. 

_I'm waiting until clap supports dynamic completions, so now they are very basic. 😰_

## Use

__First of all, please execute__
```
$ gum help
``` 
and do the same for each command like this 
```
$ gum help init
```
This will be cool, trust me.
 
**In case the help did not help:** 😢

### Init
To start working with **gum**, you may want to use `init` cmd, like this:
```BASH
# This is good anyway
$ gum init --help
$ gum init
```
You may want to save the config to another file, if so, use flag `--file/-f`

```
$ gum init --file custom-name.yaml
```

Also, you may wanna scrap the config from you gitlab installation. To be sure, you know. 

```
$ gum init -g ${GROUP_ID} ${ANOTHER_GROUP_ID} --token ${PERSONAL_GITLAB_TOKEN}
```
That's enough to beging. Now you're a gum user. What's next?

### Sync 

```BASH
$ gum sync --help
```
**Sync** will create a set of actions based on your config file, and execute them. Each action will add a new entry to **state**. State is a set of Users/Groups and their access. Save will be saved and on the next run gum after creating a new state will compare it to the previous one. 
*I've got a plan to add a flag to the **sync** command to compare with a real state from gitlab, to avoid situations when somebody's adding users via UI and gum know nothing about it.*

Examples: 
```BASH
$ gum sync --dry-run # Will just show what's gonna happen, but not really doing anything. 
$ gum sync # Will actually apply all the changes.
$ gum sync --dry-run -w # Will save a state even after a dry-run
$ gum sync --dry-run -w --state-destination state.json # Will write the state to the file ./state.json
$ gum sync -s state.json # Will take the state from the file ./state.json and apply changes.
```


### Modify your config

There are two ways of modifying the config file. 
1. Just edit a yaml file. But remember that gum only needs ID of entity to process it, names and urls exist only to let you understand what's happening. And also currently gum is not checking if an entity name corresponds to entity ID, I will do it soon. So just changing names/urls won't do anything.
2. With gum itself. There are subcommands like `users`, `groups`, `teams`, you can use them to modify your config file.

### Search

```BASH
$ gum search -h
```

If you don't know any projects/groups/users ID, you can use the `search` command. Currently it can search for users, groups and projects. After executing, you will probably see a beautiful table with result.

## What is teams?

Teams is a gum entity witch is actually just a set of permissions. It's an alternative for gitlab groups. 
For example, you can create a team `frontend-devs` and add all frontend specific projects there. And then just add a new entry to frontend developers in the config. 

When you sync config with teams, if a user is in the team, the projects to which this user us directly added and projects from the team will be concatenated. If there are the same entries there, the one with a higher access will be chosen by gum. 

### Default team

After creating a config with the `init` cmd, you will find out that it's already contains one team named `default`. Be careful with this one. All users that are defined in the config file will be automatically added to the default team. If you don't need something like that, feel free to remove it. 

---
