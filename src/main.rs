#![feature(question_mark)]

extern crate log;
extern crate env_logger;
extern crate getopts;
extern crate mdblog;

use getopts::{Options, Occur, HasArg, Matches};
use std::env;
use mdblog::Mdblog;


fn print_usage(opts: Options) {
    let brief = "\
Usage:
    mdblog init <blog>
    mdblog build [-t <theme>]
    mdblog server [-p <port>]
    mdblog -v | --version
    mdblog -h | --help\
";
    print!("{}", opts.usage(brief));
}


fn main() {
    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();

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

    let matches = match opts.parse(&args[0..]) {
        Ok(m) => m,
        Err(e) => {
            println!("{}", e);
            println!("run `mdblog -h` to get the usage.");
            ::std::process::exit(1);
        },
    };

    if matches.opt_present("h") || args.len() < 2 {
        print_usage(opts);
        ::std::process::exit(0);
    }

    if matches.opt_present("v") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        ::std::process::exit(0);
    }

    let program = &args[0];
    let command = &args[1];

    match command.as_ref() {
        "init" => init(&args),
        "build" => build(&matches),
        "server" => server(&matches),
        _ => print_usage(opts),
    }
    ::std::process::exit(0);
}


fn init(args: &Vec<String>) {
    if args.len() != 3 {
        println!("`init` subcommand requires an argument.");
        ::std::process::exit(1);
    }
    let mut root_dir = ::std::env::current_dir().unwrap();
    root_dir.push(&args[2]);
    let mb = Mdblog::new(&root_dir);
    if let Err(e) = mb.init(){
        panic!(e.to_string());
    }
}


fn build(matches: &Matches) {
    let theme = matches.opt_str("t").unwrap_or("simple".to_string());
    let root_dir = ::std::env::current_dir().unwrap();
    let mut mb = Mdblog::new(&root_dir);
    if let Err(e) = mb.build(&theme) {
        panic!(e.to_string());
    }
}


fn server(matches: &Matches) {
    println!("server command");
}
