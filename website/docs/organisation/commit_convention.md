---
sidebar_position: 2
---

# Conventional Commit Messages 

See how [a minor change](#examples) to your commit message style can make a difference. 

<pre>
git commit -m"<b><a href="#types">&lt;type&gt;</a></b></font>(<b><a href="#scopes">&lt;optional scope&gt;</a></b>): <b><a href="#description">&lt;description&gt;</a></b>" \
  -m"<b><a href="#body">&lt;optional body&gt;</a></b>" \
  -m"<b><a href="#footer">&lt;optional footer&gt;</a></b>"
</pre>

> [!TIP]
> Take a look at **[git-conventional-commits](https://github.com/qoomon/git-conventional-commits)** ; a CLI util to ensure these conventions, determine version and generate changelogs.

## Commit Message Formats

### General Commit
<pre>
<b><a href="#types">&lt;type&gt;</a></b></font>(<b><a href="#scopes">&lt;optional scope&gt;</a></b>): <b><a href="#description">&lt;description&gt;</a></b>
<sub>empty separator line</sub>
<b><a href="#body">&lt;optional body&gt;</a></b>
<sub>empty separator line</sub>
<b><a href="#footer">&lt;optional footer&gt;</a></b>
</pre>

### Initial Commit 
```
chore: init
```

### Merge Commit
<pre>
Merge branch '<b>&lt;branch name&gt;</b>'
</pre>
<sup>Follows default git merge message</sup>

### Revert Commit
<pre>
Revert "<b>&lt;reverted commit subject line&gt;</b>"
</pre>
<sup>Follows default git revert message</sup>


### Types
- Changes relevant to the API or UI:
    - `feat` Commits that add, adjust or remove a new feature to the API or UI
    - `fix` Commits that fix an API or UI bug of a preceded `feat` commit
- `refactor` Commits that rewrite or restructure code without altering API or UI behavior
    - `perf` Commits are special type of `refactor` commits that specifically improve performance
- `style` Commits that address code style (e.g., white-space, formatting, missing semi-colons) and do not affect application behavior
- `test` Commits that add missing tests or correct existing ones
- `docs` Commits that exclusively affect documentation
- `build` Commits that affect build-related components such as build tools, dependencies, project version, CI/CD pipelines, ...
- `ops` Commits that affect operational components like infrastructure, deployment, backup, recovery procedures, ...
- `chore` Miscellaneous commits e.g. modifying `.gitignore`, ...

### Scopes
The `scope` provides additional contextual information.
* The scope is an **optional** part
* Allowed scopes vary and are typically defined by the specific project
* **Do not** use issue identifiers as scopes

### Breaking Changes Indicator
- Breaking changes *should* be indicated by an `!` before the `:` in the subject line e.g. `feat(api)!: remove status endpoint`
- Breaking changes **must** be described in the [commit footer section](#footer)    

### Description
The `description` contains a concise description of the change.
- The description is a **mandatory** part
- Use the imperative, present tense: "change" not "changed" nor "changes"
  - Think of `This commit will...` or `This commit should...`
- **Do not** capitalize the first letter
- **Do not** end the description with a period (`.`)

### Body
The `body` should include the motivation for the change and contrast this with previous behavior.
- The body is an **optional** part
- Use the imperative, present tense: "change" not "changed" nor "changes"

### Footer
The `footer` should contain issue references and informations about **Breaking Changes**
- The footer is an **optional** part, except if the commit introduce breaking changes
- *Optionally* reference issue identifiers (e.g., `Closes #123`, `Fixes JIRA-456`) 
- **Breaking Changes** should start with the word `BREAKING CHANGE:` followed by space or two newlines. The rest of the commit message is then used for this.

### Versioning
- **If** your next release contains commit with...
   - **Breaking Changes** incremented the **major version**
   - **API relevant changes** (`feat` or `fix`) incremented the **minor version**
- **Else** increment the **patch version**


### Examples
- ```
  feat: add email notifications on new direct messages
  ```
- ```
  feat(shopping cart): add the amazing button
  ```
- ```
  feat!: remove ticket list endpoint

  refers to JIRA-1337

  BREAKING CHANGE: ticket endpoints no longer supports list all entities.
  ```
- ```
  fix(shopping-cart): prevent order an empty shopping cart
  ```
- ```
  fix(api): fix wrong calculation of request body checksum
  ```
- ```
  fix: add missing parameter to service call

  The error occurred due to <reasons>.
  ```
- ```
  perf: decrease memory footprint for determine unique visitors by using HyperLogLog
  ```
- ```
  build: update dependencies
  ```
- ```
  build(release): bump version to 1.0.0
  ```
- ```
  refactor: implement fibonacci number calculation as recursion
  ```
- ```
  style: remove empty line
  ```

---
  
## Git Hook Scripts to ensure commit message header format
<details>
<summary>Click to expand</summary>
   
### commit-msg Hook (local)
- Create a commit-msg hook using [git-conventional-commits cli](https://github.com/qoomon/git-conventional-commits?tab=readme-ov-file#automatically-validate-commit-message-convention-before-commit)

### pre-receive Hook (server side)
- create following file in your repository folder `.git/hooks/pre-receive`
  ```shell
  #!/usr/bin/env bash

  # Pre-receive hook that will block commits with messages that do not follow regex rule

  commit_msg_type_regex='feat|fix|refactor|style|test|docs|build'
  commit_msg_scope_regex='.{1,20}'
  commit_msg_description_regex='.{1,100}'
  commit_msg_regex="^(${commit_msg_type_regex})(\(${commit_msg_scope_regex}\))?: (${commit_msg_description_regex})\$"
  merge_msg_regex="^Merge branch '.+'\$"

  zero_commit="0000000000000000000000000000000000000000"

  # Do not traverse over commits that are already in the repository
  excludeExisting="--not --all"

  error=""
  while read oldrev newrev refname; do
    # branch or tag get deleted
    if [ "$newrev" = "$zero_commit" ]; then
      continue
    fi

    # Check for new branch or tag
    if [ "$oldrev" = "$zero_commit" ]; then
      rev_span=`git rev-list $newrev $excludeExisting`
    else
      rev_span=`git rev-list $oldrev..$newrev $excludeExisting`
    fi

    for commit in $rev_span; do
      commit_msg_header=$(git show -s --format=%s $commit)
      if ! [[ "$commit_msg_header" =~ (${commit_msg_regex})|(${merge_msg_regex}) ]]; then
        echo "$commit" >&2
        echo "ERROR: Invalid commit message format" >&2
        echo "$commit_msg_header" >&2
        error="true"
      fi
    done
  done

  if [ -n "$error" ]; then
    exit 1
  fi
  ```
* ⚠ make `.git/hooks/pre-receive` executable (unix: `chmod +x '.git/hooks/pre-receive'`)

</details>

-----
## References
- https://www.conventionalcommits.org/
- https://github.com/angular/angular/blob/master/CONTRIBUTING.md
- http://karma-runner.github.io/1.0/dev/git-commit-msg.html
<br>

- https://github.com/github/platform-samples/tree/master/pre-receive-hooks  
- https://github.community/t5/GitHub-Enterprise-Best-Practices/Using-pre-receive-hooks-in-GitHub-Enterprise/ba-p/13863
