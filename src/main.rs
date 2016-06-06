extern crate getopts;

use getopts::{Options, Occur, HasArg};
use std::env;


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
        "init" => println!("init command"),
        "build" => println!("build command"),
        "server" => println!("server command"),
        _ => print_usage(opts),
    }
    ::std::process::exit(0);
}
