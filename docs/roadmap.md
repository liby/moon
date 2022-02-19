# 0.1.0

## Projects

- [x] Can define metadata
- [x] File groups
  - [x] Configure in `project.yml`
  - [x] Deep overrides with global `project.yml`
- [x] Tasks
  - [x] Configure in `project.yml`
  - [x] Deep merges with global `project.yml`
  - [x] Supports merge strategies
- [x] Checks if affected based in a file list
- [x] Lazy loads `package.json`
- [x] Lazy loads `tsconfig.json`

## Project graph

- [x] Lazy load projects when needed
- [x] Get direct dependencies
- [x] Get direct dependents

## Tasks

- [x] Command (and type)
- [x] Args
- [x] Inputs
  - [x] Checks if affected based in a file list
  - [x] Globs
  - [x] Relative paths
  - [x] Workspace relative paths
- [x] Outputs
  - [ ] Write outputs to `.moon/out`
  - [ ] Symlink/copy outputs back to project dir
- [x] Dependencies (on other tasks)
  - [ ] Self referencing tasks (`~`)
  - [ ] Deps referencing tasks (`^`)
- [x] Environment vars
- [x] Tokens
  - [x] Expands tokens defined in configs
- [x] Can run from project root or workspace root (using `run_from_workspace_root`)

## Task runner

- [x] Sorts dep graph topologically
  - [x] Groups into batches and parallelizes
  - [x] Runs in a thread pool (via tokio)
- [x] Runs task based on `type`
- [ ] Retries when failed (using `retry_count`)
- [x] Streams output when a primary target
- [x] Buffers output when a non-primary target
- [x] Bubbles up errors
- [x] Installs npm dependencies
- [x] Syncs `package.json` and `tsconfig.json` for all projects
  - [ ] Writes JSON preserving field order

### CLI

- [x] `init` command to scafflold a new project
- [x] `project` command for displaying info
- [x] `project-graph` command for outputting DOT format
  - [ ] Spin up an interactive website with full project/task data
- [x] `setup` command for installing tools
- [x] `teardown` command for uninstalling tools
- [x] `bin` command to return tool paths
- [ ] `run` command to run targets
  - [ ] Args after `--` are passed to the underlying command
- [ ] `ci` command for smart running affected targets (below)

## CI

- [ ] Compares PR against default branch
- [ ] Runs tasks if `outputs` defined or `run_in_ci` is true
- [ ] Runs dependencies AND dependents if primary project has changed

# 0.2.0

- hashing
- caching