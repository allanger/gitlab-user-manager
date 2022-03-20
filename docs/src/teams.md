# Teams

Teams is a gum entity. It's just a set of projects and groups with an access level. 

## Format V1

Team entity is looking this.

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

### Usage

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
      groups: []
  users: 
    - id: 1
      name: frontend-dev-1
      teams: []
      groups: []
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
      groups: []
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
      groups: []
      projects: 
        - id: 1
          name: shop-frontend
          access_level: Developer
        - id: 2
          name: marketing-frontend
          access_level: Developer

  groups: []
state: ""

```