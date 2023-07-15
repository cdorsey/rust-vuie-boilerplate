use std::env;
use std::path::PathBuf;

use anyhow::anyhow;
use clap::{Parser, Subcommand};
use xshell::{cmd, Shell};

fn get_project_root() -> anyhow::Result<PathBuf> {
    let cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path = cargo_dir
        .parent()
        .ok_or(anyhow!("Cannot get project root"))?
        .to_path_buf();

    Ok(path)
}

fn build(sh: &Shell) -> anyhow::Result<()> {
    cmd!(sh, "docker-compose build").run()?;

    Ok(())
}

fn run_server(sh: &Shell) -> anyhow::Result<()> {
    cmd!(sh, "cargo run --package server").run()?;

    Ok(())
}

fn run_client(sh: &Shell) -> anyhow::Result<()> {
    let project_root = get_project_root()?;

    sh.change_dir(project_root.join("client"));
    cmd!(sh, "pnpm run dev").run()?;

    Ok(())
}

fn watch_server(sh: &Shell) -> anyhow::Result<()> {
    let subcommand = "run --package server";

    cmd!(sh, "cargo watch -x {subcommand}").run()?;

    Ok(())
}

fn watch_client(sh: &Shell) -> anyhow::Result<()> {
    let project_root = get_project_root()?;

    sh.change_dir(project_root.join("client"));

    cmd!(sh, "pnpm run watch").run()?;

    Ok(())
}

fn test(sh: &Shell) -> anyhow::Result<()> {
    cmd!(sh, "cargo test").run()?;

    Ok(())
}

fn lint(sh: &Shell) -> anyhow::Result<()> {
    let package_root = get_project_root()?;

    sh.change_dir(&package_root);

    cmd!(sh, "cargo clippy").run()?;
    cmd!(sh, "cargo fmt -- --check").run()?;

    sh.change_dir(package_root.join("client"));

    cmd!(sh, "pnpm run lint").run()?;

    Ok(())
}

fn clean(sh: &Shell) -> anyhow::Result<()> {
    let project_root = get_project_root()?;

    cmd!(sh, "cargo clean").run()?;

    sh.change_dir(project_root.join("client"));

    cmd!(sh, "pnpm run clean").run()?;

    Ok(())
}

#[derive(Subcommand, Debug)]
enum Action {
    /// Build docker image with docker-compose
    Build,
    /// Run actix server
    RunServer,
    /// Run vite dev server
    RunClient,
    /// Run actix server with file watcher (requires cargo-watch)
    WatchServer,
    /// Run vite client with file watcher (requires pnpm)
    WatchClient,
    /// Run tests for server and client
    Test,
    /// Run linters for server and client
    Lint,
    /// Clean build artifacts
    Clean,
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let sh = Shell::new()?;

    match args.action {
        Action::Build => build(&sh)?,
        Action::RunServer => run_server(&sh)?,
        Action::RunClient => run_client(&sh)?,
        Action::WatchServer => watch_server(&sh)?,
        Action::WatchClient => watch_client(&sh)?,
        Action::Test => test(&sh)?,
        Action::Clean => clean(&sh)?,
        Action::Lint => lint(&sh)?,
    }

    Ok(())
}
