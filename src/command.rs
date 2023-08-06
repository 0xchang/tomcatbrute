use clap::{value_parser, Arg, ArgMatches, Command};
use std::rc::Rc;

pub struct CmdArg {
    thread: u16,
    url: String,
}

pub trait CmdGet {
    fn get_thread(&self) -> u16;
    fn get_url(&self) -> String;
}

impl CmdGet for CmdArg {
    fn get_thread(&self) -> u16 {
        self.thread
    }
    fn get_url(&self) -> String {
        self.url.to_string()
    }
}

fn banner() {
    let logo = r###"    < tomcat brute >
     --------------
            \   ^__^
             \  (oo)\_______
                (__)\       )\/\
                    ||----w |
                    ||     ||"###;
    println!("{logo}");
}

pub fn new() -> ArgMatches {
    banner();
    let c = Command::new("tomcat brute")
        .author("0xchang")
        .version("0.0.1")
        .arg( Arg::new("thread")
        .short('t')
        .long("thread")
        .value_name("thread")
        .help("Set the number of threads(default 10)").default_value("10").value_parser(value_parser!(u16))
        ).arg(Arg::new("url")
        .short('u')
        .value_name("url")
        .help("Set URL").required(true).value_parser(value_parser!(String)))
        .after_help(
            "Longer explanation to appear after the options when displaying the help information from --help or -h",
        )
        .get_matches();
    c
}

pub fn parse_cmd(c: ArgMatches) -> Rc<CmdArg> {
    let m = CmdArg {
        thread: *c.get_one("thread").expect("thread error"),
        url: c.get_one::<String>("url").expect("url error").to_owned(),
    };
    m.into()
}
