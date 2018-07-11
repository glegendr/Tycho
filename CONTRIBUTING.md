# Branching
## Feature Branches
Feature branches are used when developing a new feature or enhancement which has the potential of a development lifespan longer than a single deployment. When starting development, the deployment in which this feature will be released may not be known. No matter when the feature branch will be finished, it will always be merged back into the master branch.

During the lifespan of the feature development, the lead should watch the master branch (network tool or branch tool in GitHub) to see if there have been commits since the feature was branched. Any and all changes to master should be merged into the feature before merging back to master; this can be done at various times during the project or at the end, but time to handle merge conflicts should be accounted for.

represents the Basecamp project to which Project Management will be tracked.

Must branch from: `master`
Must merge back into: `master`
Branch naming convention: `feat/<FEATURE_NAME>`

### Working with a feature branch
If the branch does not exist yet, create the branch locally and then push to GitHub. A feature branch should always be 'publicly' available. That is, development should never exist in just one developer's local branch.

```
$ git checkout -b feat/<FEATURE_NAME> master                 // creates a local branch for the new feature
$ git push origin feat/<FEATURE_NAME>                        // makes the new feature remotely available
```
Periodically, changes made to master (if any) should be rebased back into your feature branch.
```
$ git rebase master                                          // rebase changes from master before feature branch
```
When development on the feature is complete, the maintainer should merge changes into master.
```
$ git checkout master                                        // change to the master branch
$ git pull                                                   // get the lastversion of master
$ git merge --no-ff feat/<FEATURE_NAME>                      // makes sure to create a commit object during merge
$ git push origin master                                     // push merge changes
```
## Bug Branches
Bug branches differ from feature branches only semantically. Bug branches will be created when there is a bug on the live site that should be fixed and merged into the next deployment. For that reason, a bug branch typically will not last longer than one deployment cycle. Additionally, bug branches are used to explicitly track the difference between bug development and feature development. No matter when the bug branch will be finished, it will always be merged back into master.

Although likelihood will be less, during the lifespan of the bug development, the lead should watch the master branch (network tool or branch tool in GitHub) to see if there have been commits since the bug was branched. Any and all changes to master should be merged into the bug before merging back to master; this can be done at various times during the project or at the end, but time to handle merge conflicts should be accounted for.

represents the Basecamp project to which Project Management will be tracked.
Must branch from: `master`
Must merge back into: `master`
Branch naming convention: `bug/<BUG_NAME>`
### Working with a bug branch
If the branch does not exist yet, create the branch locally and then push to GitHub. A bug branch should always be 'publicly' available. That is, development should never exist in just one developer's local branch.
```
$ git checkout -b bug/<BUG_NAME> master                 // creates a local branch for the new bug
$ git push origin bug/<BUG_NAME>                        // makes the new bug remotely available
```
Periodically, changes made to master (if any) should be rebased back into your bug branch.
```
$ git rebase master                                          // rebase changes from master before bug branch
```
When development on the feature is complete, the maintainer should merge changes into master.
```
$ git checkout master                                        // change to the master branch
$ git pull                                                   // get the lastversion of master
$ git merge --no-ff bug/<BUG_NAME>                      // makes sure to create a commit object during merge
$ git push origin master                                     // push merge changes
```
# Format of the commit message
```
<type>: <subject>
```
Any line of the commit message cannot be longer 100 characters! This allows the message to be easier to read on github as well as in various git tools.

Subject line
Subject line contains succinct description of the change.

## Allowed <type>
- feat (feature)
- fix (bug fix)
- docs (documentation)
- style (formatting, missing semi colons, …)
- refactor
- test (when adding missing tests)
- chore (maintain)
## <subject> text
- use imperative, present tense: “change” not “changed” nor “changes”
- don't capitalize first letter
- no dot (.) at the end

## Referencing issues
Closed bugs should be listed on a separate line in the footer prefixed with "Closes" keyword like this:
```
closes #234
```
or in case of multiple issues:
```
closes #123, #245, #992
```
