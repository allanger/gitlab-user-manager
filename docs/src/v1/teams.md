# Teams

Teams is a gum entity. It's just a set of projects and groups with an access level. 

## Format

Team entity is looking like this.

```YAML
teams:
  - name: ${TEAM_NAME}
    projects: 
      - name: ${GITLAB_PROJECT_NAME}
        id: ${GITLAB_PROJECT_ID}
        access_level: ${ACCESS_LEVEL}
    groups: 
      - name: ${GITLAB_PROJECT_NAME}
        id: ${GITLAB_PROJECT_ID}
        access_level: ${ACCESS_LEVEL
       url: ${GITLAB_GROUP_URL}
```

## How it's working? 
When user is added to a team, `teams.projects` and `teams.namespaces` are being added to `users.projects` and `user.namespaces`. If the same project or namespace is defined for a team and for an user which is added to that team, the higher level will be applied. For example: 
```YAML
teams: 
  - name: team_1
    projects: 
      - id: 1
        name: project_1
        access_level: Developer
    namespaces:
      - id: 1
        name: namespace_1
        access_level: Guest
        url: https://gitlab.com/groups/namespace_1
users: 
  - id: 1
    name: User Name
    teams: 
      team_1  
    projects: 
      - id: 1
        name: project_1
        access_level: Guest
    namespaces:
      - id: 1
        name: namespace_1
        access_level: Developer
        url: https://gitlab.com/groups/namespace_1
```

After syncing this config file, `User Name` will be a `Developer` in `project_1` and in `namespace_1`. 

## Default team
Every user in the config file will be added to the `default` team by default, so be careful with it. You can simply remove it if you don't wanna use it.

## Usage
Every command has a good (more or less) description. Please use `gum help teams` and you'll be fine.

## Example

Let's assume you have a gitlab group `shop-application` with two projects `shop-frontend` and `shop-frontend` and a gitlab group `marketing-application` with `application-frontend` and `application-backend`. 
And users `frontend-dev-1`, `frontend-dev-2`, `backend-dev-1`, `backend-dev-2` and `shop-product-owner` and `shop-product-owner-2`. 

You could create a gum config like this: 

```YAML
meta:
  version: v1
config:
  teams:
    - name: default
      projects: []
      namespaces: []
  users: 
    - id: 1
      name: frontend-dev-1
      teams: []
      namespaces: []
      projects: 
        - id: 1
          name: shop-frontend
          access_level: Developer
        - id: 2
          name: marketing-frontend
          access_level: Developer
    - id: 2
      name: frontend-dev-2
      teams: []
      namespaces: []
      projects: 
        - id: 1
          name: shop-frontend
          access_level: Developer
        - id: 2
          name: marketing-frontend
          access_level: Developer
    - id: 3
      name: backend-dev-1
      teams: []
      namespaces: []
      projects: 
        - id: 3
          name: shop-backend
          access_level: Developer
        - id: 4
          name: marketing-backend
          access_level: Developer
    - id: 4
      name: backend-dev-2
      teams: []
      namespaces: []
      projects: 
        - id: 3
          name: shop-backend
          access_level: Developer
        - id: 4
          name: marketing-backend
          access_level: Developer
    - id: 5
      name: shop-product-owner
      teams: []
      namespaces: []
      projects: 
        - id: 1
          name: shop-frontend
          access_level: Maintainer
        - id: 3
          name: shop-backend
          access_level: Maintainer
  groups: []
state: ""
```
But to avoid a lot of repetitions and make yaml file a little bit more compact, you could use `teams` like this: 
```YAML
meta:
  version: v1
config:
  teams:
    - name: frontend
      namespaces: []
      projects: 
        - id: 1
          name: shop-frontend
          access_level: Developer
        - id: 2
          name: marketing-frontend
          access_level: Developer
    - name: backend
      namespaces: []
      projects: 
        - id: 3
          name: shop-backend
          access_level: Developer
        - id: 4
          name: marketing-backend
          access_level: Developer
    - name: shop-owners
      namespaces: []
      projects: 
        - id: 1
          name: shop-frontend
          access_level: Maintainer
        - id: 3
          name: shop-backend
          access_level: Maintainer
  users: 
    - id: 1
      name: frontend-dev-1
      teams: 
        - frontend
      namespaces: []
      projects: []
    - id: 2
      name: frontend-dev-2
      teams: 
        - frontend
      namespaces: []
      projects: []
    - id: 3
      name: backend-dev-1
      teams: 
        - backend
      namespaces: []
      projects: []
    - id: 4
      name: backend-dev-2
      teams: 
        - backend
      namespaces: []
      projects: []
    - id: 5
      name: shop-product-owner
      teams: 
        - shop-owners
      namespaces: []
      projects: []
  groups: []
state: ""
```


