# `cf`: a conda-forge CLI

Tasks:

- A `feedstock` subcommand (maybe accept alias `f`, `fs`): clone and start working on a given feedstock. The argument should just be the project name sans-"feedstock", not the whole URL or user/repo pair. So given a name e.g. `numpy`,  clone `https://github.com/conda-forge/numpy-feedstock`.
    - It could also have subsubcommands (maybe even all the conda-forge-admin commands)
      - "update version"
      - "rerender"
      - "lint"
      - add a migrator
      - `config`: conda-forge.yml shortcuts, maybe TUI with dropdowns and whatnot
    - Once ready, a PR should be opened (after forking and updating the branch remote) or updated for that branch.
    - We should also be able to work an open PR created by someone else (including bots). Directly on branch if a maintainer, or by forking first.
    - More tasks:
        - Check versions
- `staged-recipes` tooling: create new recipe with a cookiecutter template or something like that.
- A `status` subcommand for migrations, version updates, etc.
