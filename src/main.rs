extern crate log;
extern crate env_logger;
#[macro_use]
extern crate structopt;
extern crate failure;
extern crate mdblog;

use std::env;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use mdblog::{Mdblog, Result, log_error};

#[derive(StructOpt, Debug)]
#[structopt(name = "mdblog")]
/// static site generator from markdown files
enum Opt {
    #[structopt(name = "init")]
    /// Initialize the blog directory layout
    Init {
        /// the blog directory name
        name: String,
    },
     #[structopt(name = "new")]
    /// Create a blog post
    New {
        #[structopt(short = "t", long = "tag", default_value = "blog")]
        /// Post tags
        tags: Vec<String>,
        /// Post path relative to blog `posts` directory
        path: PathBuf,
    },
    #[structopt(name = "build")]
    /// Build the blog static files
    Build,
    #[structopt(name = "serve")]
    /// Serve the blog, rebuild on change
    Serve {
        #[structopt(short = "p", long = "port", default_value = "5000")]
        /// Serve the blog at http://127.0.0.1:<port>
        port: u16,
    },
    #[structopt(name = "theme")]
    /// Blog theme operations
    Theme(SubCommandTheme),
}


#[derive(StructOpt, Debug)]
enum SubCommandTheme {
    #[structopt(name = "list")]
    /// list blog themes
    List,
    #[structopt(name = "new")]
    /// Create a new theme
    New {
        /// theme name
        name: String,
    },
     #[structopt(name = "delete")]
    /// Delete a theme
    Delete {
        /// theme name
        name: String,
    },
    #[structopt(name = "set")]
    /// Set blog use the theme
    Set {
        /// theme name
        name: String,
    },
}


fn main() {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .init();

    let opt = Opt::from_args();
    let res = match opt {
        Opt::Init {ref name} => init(name),
        Opt::New {ref tags, ref path} => new(path, tags),
        Opt::Build => build(),
        Opt::Serve { port } => serve(port),
        Opt::Theme(ref subcmd) => theme(subcmd),
    };

    if let Err(ref e) = res {
        log_error(e);
        std::process::exit(1);
    }
}

fn init(name: &str) -> Result<()> {
    let root_dir = env::current_dir()?.join(name);
    let mut mb = Mdblog::new(root_dir)?;
    mb.init()?;
    Ok(())
}

fn new(path: &Path, tags: &Vec<String>) -> Result<()> {
    let root_dir = env::current_dir()?;
    let mut mb = Mdblog::new(&root_dir)?;
    mb.load_customize_settings()?;
    mb.create_post(path, tags)?;
    Ok(())
}

fn build() -> Result<()> {
    let root_dir = env::current_dir()?;
    let mut mb = Mdblog::new(&root_dir)?;
    mb.load()?;
    mb.build()?;
    Ok(())
}

fn serve(port: u16) -> Result<()> {
    let root_dir = env::current_dir()?;
    let mut mb = Mdblog::new(&root_dir)?;
    mb.load()?;
    mb.build()?;
    mb.serve(port)?;
    Ok(())
}

fn theme(cmd: &SubCommandTheme) -> Result<()> {
    let root_dir = env::current_dir()?;
    let mut mb = Mdblog::new(&root_dir)?;
    mb.load_customize_settings()?;

    match cmd {
        &SubCommandTheme::List => mb.list_blog_theme()?,
        &SubCommandTheme::New { ref name } => mb.create_blog_theme(name)?,
        &SubCommandTheme::Delete { ref name } => mb.delete_blog_theme(name)?,
        &SubCommandTheme::Set { ref name } => mb.set_blog_theme(name)?,
    }
    Ok(())
}