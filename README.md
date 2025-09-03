<h1 align="center">rust-template</h1>

<p align="center">
  <a href="#about">About</a> •
  <a href="#features">Features</a> •
  <a href="#dependencies">Dependencies</a> •
  <a href="#building">Building</a> •
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#configuration">Configuration</a>
</p>

---

## About

`rust-template` is a custom, opinionated template for creating new Rust projects.

❗ The following **must be changed**, if you decide to use this template ❗

- change `[package]` fields in `Cargo.toml` to match your project
- change paths in the `Dockerfile` to match your project and release binary
- change the license holder within `LICENSE` to match your project
- change the banner in `src/main.rs` to match your project
- generate a PAT and add it as the `GH_TOKEN` secret to the repository
- possibly change the GitHub Actions workflow to match your project
- add the `renovate` GitHub App to the repository if you want automated dependency updates
- change `renovate.json` if you want to customize the dependency update logic. By default, it uses `develop` as its base branch.

You should also adhere to the `conventional commits` standard for commit messages, as this is used by `semantic-release` to determine the next version.

## Features

### rust

- `figment` for configuration
- `tokio` and `async-trait` for async handling
- `tracing` for logging/tracing
- `eyre` for error handling
- `serde` for serialization/deserialization

### CI/CD

- `GitHub Actions` for CI/CD
- `semantic-release` for versioning
- `renovate` for dependency updates

## Dependencies

> If there are any third-party dependencies, they should be listed here.

## Building

> Describe how to build the project.

## Installation

> Describe how to install the project.

## Usage

> Describe how to use the project.

## Configuration

```toml
[general]
log_level = "info"
show_banner = true
```

> Describe how to configure the project.

## Shoutout

> Give a shoutout to any libraries or projects that inspired this one.
