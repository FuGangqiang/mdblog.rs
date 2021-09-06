use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

use clap::Clap;
use log::error;
use mdblog::{Mdblog, Result};

#[derive(Clap, Debug)]
#[clap(name = "mdblog")]
/// static site generator from markdown files
enum Opt {
    #[clap(name = "init")]
    /// Initialize the blog directory layout
    Init {
        /// the blog directory name
        name: String,
    },
    #[clap(name = "new")]
    /// Create a blog post
    New {
        #[clap(short = 't', long = "tag", default_value = "")]
        /// Post tags
        tags: Vec<String>,
        #[clap(parse(from_os_str))]
        /// Post path relative to blog `posts` directory
        path: PathBuf,
    },
    #[clap(name = "build")]
    /// Build the blog static files
    Build,
    #[clap(name = "serve")]
    /// Serve the blog, rebuild on change
    Serve {
        #[clap(short = 'p', long = "port", default_value = "5000")]
        /// Serve the blog at http://127.0.0.1:<port>
        port: u16,
    },
    #[clap(name = "theme")]
    /// Blog theme operations
    Theme {
        #[clap(subcommand)]
        subcmd: SubCommandTheme,
    },
}

#[derive(Clap, Debug)]
enum SubCommandTheme {
    #[clap(name = "list")]
    /// list blog themes
    List,
    #[clap(name = "new")]
    /// Create a new theme
    New {
        /// theme name
        name: String,
    },
    #[clap(name = "delete")]
    /// Delete a theme
    Delete {
        /// theme name
        name: String,
    },
    #[clap(name = "set")]
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

    let opt = Opt::parse();
    let res = match opt {
        Opt::Init { ref name } => init(name),
        Opt::New { ref tags, ref path } => new(path, tags),
        Opt::Build => build(),
        Opt::Serve { port } => serve(port),
        Opt::Theme{ ref subcmd } => theme(subcmd),
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

fn serve(port: u16) -> Result<()> {
    let root_dir = env::current_dir()?;
    let mut mb = Mdblog::new(&root_dir)?;
    mb.load_customize_settings()?;
    mb.serve(port)?;
    Ok(())
}

fn theme(cmd: &SubCommandTheme) -> Result<()> {
    let root_dir = env::current_dir()?;
    let mut mb = Mdblog::new(&root_dir)?;
    mb.load_customize_settings()?;

    match *cmd {
        SubCommandTheme::List => mb.list_blog_theme()?,
        SubCommandTheme::New { ref name } => mb.create_blog_theme(name)?,
        SubCommandTheme::Delete { ref name } => mb.delete_blog_theme(name)?,
        SubCommandTheme::Set { ref name } => mb.set_blog_theme(name)?,
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
