# GUM
> or just Gitlab User Manager

[![Version build](https://github.com/allanger/gitlab-user-manager/actions/workflows/build-version.yaml/badge.svg)](https://github.com/allanger/gitlab-user-manager/actions/workflows/build-version.yaml)
[![Version container](https://github.com/allanger/gitlab-user-manager/actions/workflows/container-version.yaml/badge.svg)](https://github.com/allanger/gitlab-user-manager/actions/workflows/container-version.yaml)
[![Stable container](https://github.com/allanger/gitlab-user-manager/actions/workflows/container-stable.yaml/badge.svg)](https://github.com/allanger/gitlab-user-manager/actions/workflows/container-stable.yaml)

## What's this?
![gum](./assets/gum.jpg)

As I said, it's just a gitlab user manager. The better question, why do you want to use it? **Because gitlab user management is such a mess**!

What? You don't think so? Well...

I will try to explain my point after I describe what this project exists for. 

In short, using **gum** you can control who and how can access your projects and groups with the everything-as-code approach. Just create a config file and sync it. 

Back to GitLab.

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
      - **Project**: Crypto Exchanger Scripts
    - **Project**: Crypto Exchanger Documentation
  - **SubGroup**: Just a Sock Shop
    - **Project**: Sock Shop Web
    - **Project**: Sock Shop Api
    - **SubGroup**: Crypto Exchanger Devops:
      - **Project**: Crypto Exchanger Infrastructure
      - **Project**: Crypto Exchanger Scripts
    - **Project**: Crypto Exchanger Documentation

Already tough, nah? 
And now you have to give users access to you groups and projects. I see several ways of doing that:
1. You can just add them to the main parent group
2. You can add them directly to projects
3. You can add them to subgroups
4. You can create one more subgroups with another subgroups level, add those subgroups to projects and add users just there. 

Now pros and cons
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



## Install 
### Download 

Get executable from github releases

Prebuilt binaries exist for **Linux x86_64** and **MacOS arm64** and **x86_64**
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

**Note:** Images with versions 0.0.1 and 0.0.2 are broken, because the workdir is set to `/bin`

### Build from source
1. Build binary
```
$ cargo build --release
``` 
2. Run `gum help`

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
 
**In case the help did not help:**

😢

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
