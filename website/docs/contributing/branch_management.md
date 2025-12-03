---
sidebar_position: 1
---

# Branch management

## Workflow example
Context : You want to create a new feature / fix a bug  
Step 1 : create a new branch from dev named after what you want to do ex: `feat/static_analysys/support_for_large_files`  
Step 2 : when the feature is implemented create a pull request to dev  
Step 3 : ask for somebody to review your pull request  
Step 4 : when your pr is approved take a look at the result of the github actions and advise what to do next

# Commit Norm
Commits should follow the [git karma norm](https://karma-runner.github.io/6.4/dev/git-commit-msg.html).   
In the context of this project scope can be defined as what part of the project the commit is for:
* static analysys
* dynamic analysys
* gui
* ...

# main
### purpose :
Production branch, represent the latest stable version of the project.  
### workflow :
Every change to main should come with a version release.  
### commit policy : 
No commits should ever be made on main.  
### CI/CD : 
* Integration test => blocking
* Performance test => blocking
* Security => blocking
### pull-request restrcitions
2 reviews are required to validate a pull-request to main

# dev
### purpose :
Development branch, meant as a base to branch off for new feature development.
### workflow :
When the branch is stable and all the features of a milestone are implemented a merge on main should be done.
### commit policy : 
Commits on dev should be avoided as much as possible prefer making a feature or fix branch instead.
### CI/CD : 
* Coding style => blocking
* Compilation => blocking
* Security => not blocking
### pull-request restrcitions
1 review is required to validate a pull-request to dev

# feature / fix
### purpose :
Used to create or fix a feature.
### workflow :
A branch should only contain one feature or fix, don't hesitate to create a lot of branches.
### commit policy : 
Commits should follow the [git karma norm](https://karma-runner.github.io/6.4/dev/git-commit-msg.html) 
### CI/CD : 
* Coding style => not blocking

# hotfix
### purpose :  
Used to patch a breaking bug or security issue on main.
### workflow :  
When the fix is done, main should be merged into dev. This is the only time when main in merged into another branch.
### CI/CD (manual trigger before merging with main): 
* Coding style
* Compilation
* Security

# Branch naming convention

When creating a new branch the name should be as follow :   
`<branch_type>/<scope>/<feature>`  

