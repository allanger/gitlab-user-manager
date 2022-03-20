# CMD: `init`

## Usage:
```
$ gum init --help
```

You can init an empty file with 
```
$ gum init
```
Output: 
```
Config file is generated, check it out
 $ cat gum-config.yaml
```

This command generated an empty config file. 
```
$ cat gum-config.yaml 
---
meta:
  version: v1
config:
  teams:
    - name: default
      projects: []
      groups: []
  users: []
  groups: []
state: ""
```  
