#![allow(unused_variables)]

extern crate env_logger;
extern crate getopts;
extern crate log;
extern crate failure;
extern crate mdblog;

use std::env;

use getopts::{HasArg, Matches, Occur, Options};

use mdblog::{Mdblog, Result, Error, print_error};

fn print_usage_and_exit(opts: &Options, exit_code: i32) -> ! {
    let brief = "\
Usage:
    mdblog init <blog>
    mdblog build
    mdblog serve [-p <port>]
    mdblog -v | --version
    mdblog -h | --help\
";
    if exit_code == 0 {
        print!("{}", opts.usage(brief));
    } else {
        eprint!("{}", opts.usage(brief));
    }
    ::std::process::exit(exit_code);
}

fn main() {
    env_logger::init();

    let args: Vec<_> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Display this message");
    opts.optflag("v", "version", "Print version info and exit");
    opts.opt("p",
             "port",
             "Serve with port number",
             "<port>",
             HasArg::Yes,
             Occur::Optional);

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(why) => {
            eprintln!("{}", why);
            eprintln!("run `mdblog -h` to get the usage.");
            ::std::process::exit(1);
        },
    };

    if matches.opt_present("h") {
        print_usage_and_exit(&opts, 0);
    } else if matches.opt_present("v") {
        println!("mdblog {}", env!("CARGO_PKG_VERSION"));
        ::std::process::exit(0);
    } else if matches.free.len() < 1 {
        print_usage_and_exit(&opts, 2);
    }

    let res = match matches.free[0].as_ref() {
        "init" => init(&matches),
        "build" => build(&matches),
        "serve" => serve(&matches),
        _ => print_usage_and_exit(&opts, 3),
    };

    if let Err(ref e) = res {
        print_error(e);
        ::std::process::exit(1);
    }
}

fn init(matches: &Matches) -> Result<()> {
    if matches.free.len() != 2 {
        return Err(Error::Argument("`init` subcommand requires one argument.".to_string()));
    }
    let dir = env::current_dir()?.join(&matches.free[1]);
    let mut mb = Mdblog::new(dir)?;
    mb.init()?;
    Ok(())
}

fn build(matches: &Matches) -> Result<()> {
    let root_dir = env::current_dir()?;
    let mut mb = Mdblog::new(&root_dir)?;
    mb.load()?;
    mb.build()?;
    Ok(())
}

fn serve(matches: &Matches) -> Result<()> {
    let port = matches.opt_str("port")
                      .unwrap_or("5000".to_string())
                      .parse()?;
    let root_dir = env::current_dir()?;
    let mut mb = Mdblog::new(&root_dir)?;
    mb.load()?;
    mb.build()?;
    mb.serve(port)?;
    Ok(())
}
