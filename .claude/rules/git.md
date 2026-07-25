# Git

Not scoped to paths; commits happen everywhere, so this loads every session.

## Commit messages

Every commit message subject follows Conventional Commits
(https://gist.github.com/qoomon/5dfcdf8eec66a051ecd85625518cfd13):

    <type>(<optional scope>): <description>

### Types

- **feat** — adds, adjusts, or removes a feature of the API or UI
- **fix** — fixes an API or UI bug of a preceding feat commit
- **refactor** — rewrites or restructures code without altering API or UI
  behavior
- **perf** — a refactor that specifically improves performance
- **style** — code style only (white-space, formatting, missing semi-colons);
  does not affect behavior
- **test** — adds missing tests or corrects existing ones
- **docs** — exclusively affects documentation
- **build** — affects build components: build tools, dependencies, project
  version, ...
- **ops** — affects operational aspects: infrastructure, deployment scripts,
  CI/CD pipelines, backups, monitoring, recovery, ...
- **chore** — miscellaneous tasks: initial commit, modifying .gitignore, ...

### Description

- Mandatory. One predicate, two at most — if it needs "and", consider
  splitting the commit.
- Imperative, present tense: "change", not "changed" nor "changes". Think
  *this commit will...*
- Do not capitalize the first letter.
- Do not end with a period.

Enforced by: nothing yet. This is style guidance, not a gate. If it should
become a gate, that is a commit-msg hook or a CI check on PR titles — propose
it, do not assume it.
