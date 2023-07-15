# Vue + Actix + Docker boilerplate

Functional boilerplate code for a web app using a client-side rendered Vue app for the front end, and actix web server for both API calls and static file handling. Includes everything needed to test, lint, build, and run the app, as well as a multi-stage Docker build.

## Requirements

- Rust toolchain https://www.rust-lang.org/tools/install
- NodeJS https://nodejs.org/en
- pnpm https://pnpm.io/installation
- Docker https://docs.docker.com/get-docker/
- Docker Compose https://docs.docker.com/compose/install/

## To Use

Clone this repo, remove the existing local git repo, then create your own

```sh
git clone --depth 1 https://github.com/cdorsey/rust-vue-boilerplate.git <your-project>
cd <your-project>
rm -rf .git
git init
```

## Scripts

This template makes use of the [cargo xtask](https://github.com/matklad/cargo-xtask) workflow. You can view existing commands with `cargo xtask help`. Scripts are defined [here](xtask/src/main.rs).
