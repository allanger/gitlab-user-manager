# CMD: `init`

## Usage:
```
$ gum init --help
```

- You can init an empty file with 
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
- Or you can create a config file that will describe you current Gitlab state
  Let's assume you have 3 groups with that you wanna use `gum`. Their IDs are 1, 2, 3 and 4.
  You can run this:
  ```
  $ gum init -g 1 2 3 4
  ```
  Output: 
  ```
  Config file is generated, check it out
   $ cat gum-config.yaml
  ```
  
  If inside this group the access level was already set anyhow you will see it in `gum-config.yaml`

- Also, you can use another file name using the flag `-f / --file` 
  ```
  $ gum init -f another-file-name.yaml
  
  Config file is generated, check it out
    $ cat another-file-name.yaml
  ```
