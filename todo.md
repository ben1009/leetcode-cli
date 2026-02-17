# TODO

## Fix Lints

- [ ] Fix all clippy warnings
- [ ] Add `#![warn(clippy::all)]` to main.rs
- [ ] Run `cargo clippy -- -D warnings` in CI
- [ ] Fix unused imports
- [ ] Fix unused variables
- [ ] Fix redundant clones
- [ ] Fix needless borrows

## Add GitHub Actions

- [ ] Create `.github/workflows/ci.yml`
- [ ] Add build job (stable, beta, nightly)
- [ ] Add test job
- [ ] Add clippy check
- [ ] Add rustfmt check
- [ ] Add release workflow
- [ ] Add dependabot for dependency updates

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
