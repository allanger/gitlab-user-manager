# GUM

[![Maintainer](https://github.com/allanger/gitlab-user-manager/actions/workflows/container-version.yaml/badge.svg)](https://img.shields.io/badge/maintainer-allanger-blue)
[![Version build](https://github.com/allanger/gitlab-user-manager/actions/workflows/build-version.yaml/badge.svg)](https://github.com/allanger/gitlab-user-manager/actions/workflows/build-version.yaml)
[![Version container](https://github.com/allanger/gitlab-user-manager/actions/workflows/container-version.yaml/badge.svg)](https://github.com/allanger/gitlab-user-manager/actions/workflows/container-version.yaml)
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
