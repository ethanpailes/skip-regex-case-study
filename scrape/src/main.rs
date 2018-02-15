
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate docopt;
extern crate regex;

use docopt::Docopt;
use regex::bytes::Regex;

use std::io::{BufRead, BufReader};
use std::fs::File;

const USAGE: &'static str = "
Usage: scrape [-d | -s | -t] <log-file>
       scrape -h

Options:
    -d, --debug     Run with standard regex rather than skip regex.
    -s, --standard  Run the filter routine written with idiomatic standard regex.
    -h, --help      Print this help message.
";

#[derive(Deserialize)]
struct Args {
    arg_log_file: String,

    flag_debug: bool,
    flag_standard: bool,

    // docopt has a builtin help flag handler if you include it in your
    // usage message.
    // flag_help: bool,
}

macro_rules! regex {
    ($re:expr, $debug:expr) => {{
        use regex::bytes::Regex;
        use regex::internal::ExecBuilder;

        if $debug {
            Regex::new($re).unwrap()
        } else {
            ExecBuilder::new($re)
                .skip_backtrack().only_utf8(false).build()
                .map(regex::bytes::Regex::from)
                .unwrap()
        }
    }}
}

/// A little script to generate a histogram of messages published in each
/// second.
fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut no_lines = 0;
    let mut no_matching_lines = 0;

    // let msg_line = r"\[(.*)\] TRACE Appended message set to log .* with first offset: ([0-9]*).*";
    let msg_line = r"\[(.*)\] .* with first offset: ([0-9]*).*";

    let skippable_re =
        regex!(&format!("{}{}", msg_line, r"|.*"), args.flag_debug);
    let idiomatic_re = Regex::new(msg_line).unwrap();

    let file = File::open(&args.arg_log_file).expect("cannot open log file");
    let file = BufReader::new(file);
    for line in file.lines().filter_map(|result| result.ok()) {
        no_lines += 1;
        let data = if args.flag_standard {
                idiomatic_re_scrape_msg(&idiomatic_re, line.as_bytes())
            } else {
                skippable_scrape_msg(&skippable_re, line.as_bytes())
            };

        if data.is_some() {
            no_matching_lines += 1;
        }
    } 

    println!("{}/{} lines match in the file", no_matching_lines, no_lines);
}

fn skippable_scrape_msg<'a>(re: &Regex, line: &'a [u8])
    -> Option<(&'a [u8], &'a [u8])>
{
    let caps = re.captures(line).unwrap();
    match (caps.get(1), caps.get(2)) {
        (Some(date), Some(thing)) => Some((date.as_bytes(), thing.as_bytes())),
        _ => None
    }
}

fn idiomatic_re_scrape_msg<'a>(re: &Regex, line: &'a [u8])
    -> Option<(&'a [u8], &'a [u8])>
{
    re.captures(line).and_then(|caps| {
        match (caps.get(1), caps.get(2)) {
            (Some(date), Some(thing)) =>
                Some((date.as_bytes(), thing.as_bytes())),
            _ => None
        }
    })
}
