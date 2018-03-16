#![allow(unused_variables)]
#[allow(unused_doc_comment)]

extern crate env_logger;
extern crate getopts;
extern crate log;
extern crate mdblog;

use getopts::{HasArg, Matches, Occur, Options};
use mdblog::{Mdblog, Result};
use std::env;

fn print_usage_and_exit(opts: &Options, exit_code: i32) -> ! {
    let brief = "\
Usage:
    mdblog init <blog>
    mdblog build
    mdblog server [-p <port>]  # unimplemented
    mdblog -v | --version
    mdblog -h | --help\
";
    if exit_code == 0 {
        eprint!("{}", opts.usage(brief));
    } else {
        print!("{}", opts.usage(brief));
    }
    ::std::process::exit(exit_code);
}

fn main() {
    env_logger::init().expect("env_logger init error");

    let args: Vec<_> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Display this message");
    opts.optflag("v", "version", "Print version info and exit");
    opts.opt("p",
             "port",
             "Server with port number",
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
        "server" => server(&matches),
        _ => print_usage_and_exit(&opts, 3),
    };

    if let Err(ref e) = res {
        eprintln!("error: {}", e);

        for e in e.iter().skip(1) {
            eprintln!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            eprintln!("backtrace: {:?}", backtrace);
        }
        ::std::process::exit(1);
    }
}

fn init(matches: &Matches) -> Result<()> {
    if matches.free.len() != 2 {
        panic!("`init` subcommand requires one argument.");
    }
    let dir = env::current_dir().unwrap().join(&matches.free[1]);
    let mut mb = Mdblog::new(dir).unwrap();
    mb.init().unwrap();
    Ok(())
}

fn build(matches: &Matches) -> Result<()> {
    let root_dir = env::current_dir().unwrap();
    let mut mb = Mdblog::new(&root_dir).unwrap();
    mb.load().unwrap();
    mb.build().unwrap();
    Ok(())
}

fn server(matches: &Matches) -> Result<()> {
    println!("server command");
    Ok(())
}
