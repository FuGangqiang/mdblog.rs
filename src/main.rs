#![allow(unused_variables)]

extern crate log;
extern crate env_logger;
extern crate getopts;
extern crate mdblog;

use getopts::{Options, Occur, HasArg, Matches};
use mdblog::Mdblog;
use std::env;
use std::process::exit;


fn print_usage(opts: Options) {
    let brief = "\
Usage:
    mdblog init <blog> [-t <theme>]
    mdblog build [-t <theme>]
    mdblog server [-p <port>]
    mdblog -v | --version
    mdblog -h | --help\
";
    print!("{}", opts.usage(brief));
}


fn main() {
    env_logger::init().expect("env_logger init error");

    let args: Vec<_> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Display this message");
    opts.optflag("v", "version", "Print version info and exit");
    opts.opt("t",
             "theme",
             "Build with specified theme",
             "<theme>",
             HasArg::Yes,
             Occur::Optional);
    opts.opt("p",
             "port",
             "Server with port number",
             "<port>",
             HasArg::Yes,
             Occur::Optional);

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(why) => {
            println!("{}", why);
            println!("run `mdblog -h` to get the usage.");
            exit(1);
        },
    };

    if matches.opt_present("h") {
        print_usage(opts);
        exit(0);
    } else if matches.opt_present("v") {
        println!("mdblog {}", env!("CARGO_PKG_VERSION"));
        exit(0);
    } else if matches.free.len() < 1 {
        print_usage(opts);
        exit(2);
    }

    match matches.free[0].as_ref() {
        "init" => init(&matches),
        "build" => build(&matches),
        "server" => server(&matches),
        _ => print_usage(opts),
    }
    exit(0);
}


fn init(matches: &Matches) {
    if matches.free.len() != 2 {
        panic!("`init` subcommand requires one argument.");
    }
    let dir = env::current_dir()
        .unwrap()
        .join(&matches.free[1]);
    let mb = Mdblog::new(dir);
    let theme = matches.opt_str("theme");
    match mb.init(theme) {
        Ok(_) => exit(0),
        Err(why) => panic!(why.to_string()),
    }
}


fn build(matches: &Matches) {
    let root_dir = env::current_dir().unwrap();
    let mut mb = Mdblog::new(&root_dir);
    let theme = matches.opt_str("theme");
    mb.load_config().unwrap();
    match mb.build(theme) {
        Ok(_) => exit(0),
        Err(why) => panic!(why.to_string()),
    }
}


fn server(matches: &Matches) {
    println!("server command");
}
