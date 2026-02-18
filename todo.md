# TODO

## Fix Lints

- [x] Fix all clippy warnings
- [x] Add `#![warn(clippy::all)]` to main.rs
- [x] Run `cargo clippy -- -D warnings` in CI
- [x] Fix unused imports
- [x] Fix unused variables
- [x] Fix redundant clones
- [x] Fix needless borrows

## Add GitHub Actions

- [x] Create `.github/workflows/ci.yml`
- [x] Add build job (stable, beta, nightly)
- [x] Add test job
- [x] Add clippy check
- [x] Add rustfmt check
- [x] Add release workflow
- [x] Add dependabot for dependency updates

## CI Workflow Template

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --release

  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test

  clippy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - run: cargo clippy -- -D warnings

  fmt:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo fmt -- --check
```
