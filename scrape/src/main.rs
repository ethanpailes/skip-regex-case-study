
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

struct Stats {
    appends: usize,
    isr_change_props: usize,
}
impl Stats {
    fn new() -> Self {
        Stats {
            appends: 0,
            isr_change_props: 0,
        }
    }
}

/// A little script to generate a histogram of messages published in each
/// second.
fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut no_lines = 0;

    let append_line = r"\[(.*)\] .* with first offset: ([0-9]*).*";
    let change_prop = r"\[(.*)\] .* scheduled task 'isr-change-propagation'.*";

    let skippable_re =
        regex!(&format!("{}|{}|.*", append_line, change_prop),
                args.flag_debug);

    let idiomatic_res = vec![
            Regex::new(append_line).unwrap(),
            Regex::new(change_prop).unwrap(),
        ];

    let mut stats = Stats::new();

    let file = File::open(&args.arg_log_file).expect("cannot open log file");
    let file = BufReader::new(file);
    for line in file.lines().filter_map(|result| result.ok()) {
        no_lines += 1;
        if args.flag_standard {
            idiomatic_re_scrape_msg(&idiomatic_res, line.as_bytes(), &mut stats)
        } else {
            skippable_scrape_msg(&skippable_re, line.as_bytes(), &mut stats)
        }

    } 

    println!("{}/{} append events", stats.appends, no_lines);
    println!("{}/{} 'isr-change-propagation' events",
                stats.isr_change_props, no_lines);
}

fn skippable_scrape_msg<'a>(re: &Regex, line: &'a [u8], stats: &mut Stats)
{
    let caps = re.captures(line).unwrap();

    if caps.get(1).is_some() {
        stats.appends += 1;
    } else if caps.get(3).is_some() {
        stats.isr_change_props += 1;
    }
}

fn idiomatic_re_scrape_msg<'a>(res: &[Regex], line: &'a [u8], stats: &mut Stats)
{
    match res[0].captures(line) {
        Some(caps) => 
            match caps.get(1) {
                Some(_) => {
                    stats.appends += 1;
                }
                None => (),
            },
        None => (),
    }

    match res[1].captures(line) {
        Some(caps) => 
            match caps.get(1) {
                Some(_) => {
                    stats.isr_change_props += 1;
                }
                None => (),
            },
        None => (),
    }
}
