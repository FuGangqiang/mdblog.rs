use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use log::error;
use mdblog::{Mdblog, Result};

/// static site generator from markdown files
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    cmd: CliCommand,
}

#[derive(Subcommand, Debug)]
enum CliCommand {
    /// Initialize the blog directory layout
    Init {
        /// the blog directory name
        name: String,
    },
    /// Create a blog post
    New {
        #[clap(name="tag", short, long = "tag", default_value = "")]
        /// Post tags
        tags: Vec<String>,
        /// Post path relative to blog `posts` directory
        path: PathBuf,
    },
    /// Build the blog static files
    Build,
    /// Serve the blog, rebuild on change
    Serve {
        #[clap(long, default_value = "127.0.0.1")]
        /// Serve the blog at <host>
        host: String,
        #[clap(short, long, default_value = "5000")]
        /// Serve the blog at <port>
        port: u16,
    },
    /// Blog theme operations
    Theme {
        #[clap(subcommand)]
        cmd: ThemeCommand,
    },
}

#[derive(Parser, Debug)]
enum ThemeCommand {
    /// list blog themes
    List,
    /// Create a new theme
    New {
        /// theme name
        name: String,
    },
    /// Delete a theme
    Delete {
        /// theme name
        name: String,
    },
    /// Set blog use the theme
    Set {
        /// theme name
        name: String,
    },
}

fn main() {
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let cli = Cli::parse();
    let res = match cli.cmd {
        CliCommand::Init { ref name } => init(name),
        CliCommand::New { ref tags, ref path } => new(path, tags),
        CliCommand::Build => build(),
        CliCommand::Serve { host, port } => serve(host, port),
        CliCommand::Theme { ref cmd } => theme(cmd),
    };

    if let Err(ref e) = res {
        log_error_chain(e);
        std::process::exit(1);
    }
}

fn init(name: &str) -> Result<()> {
    let root_dir = env::current_dir()?.join(name);
    let mut mb = Mdblog::new(root_dir)?;
    mb.init()?;
    Ok(())
}

fn new(path: &Path, tags: &[String]) -> Result<()> {
    let root_dir = env::current_dir()?;
    let mut mb = Mdblog::new(&root_dir)?;
    mb.load_customize_settings()?;
    mb.create_post(path, tags)?;
    Ok(())
}

fn build() -> Result<()> {
    let root_dir = env::current_dir()?;
    let mut mb = Mdblog::new(&root_dir)?;
    mb.load_customize_settings()?;
    mb.build()?;
    Ok(())
}

fn serve(host: String, port: u16) -> Result<()> {
    let root_dir = env::current_dir()?;
    let mut mb = Mdblog::new(&root_dir)?;
    mb.load_customize_settings()?;
    mb.serve(host, port)?;
    Ok(())
}

fn theme(cmd: &ThemeCommand) -> Result<()> {
    let root_dir = env::current_dir()?;
    let mut mb = Mdblog::new(&root_dir)?;
    mb.load_customize_settings()?;

    match *cmd {
        ThemeCommand::List => mb.list_blog_theme()?,
        ThemeCommand::New { ref name } => mb.create_blog_theme(name)?,
        ThemeCommand::Delete { ref name } => mb.delete_blog_theme(name)?,
        ThemeCommand::Set { ref name } => mb.set_blog_theme(name)?,
    }
    Ok(())
}

fn log_error_chain(mut e: &dyn Error) {
    error!("error: {}", e);
    while let Some(source) = e.source() {
        error!("caused by: {}", source);
        e = source;
    }
}
