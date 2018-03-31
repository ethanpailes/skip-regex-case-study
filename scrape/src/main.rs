
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate docopt;
extern crate regex;

use docopt::Docopt;
use regex::bytes::Regex;

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp;
use std::collections::HashMap;

const USAGE: &'static str = "
Usage: scrape [options] <log-file>
       scrape -h

Options:
    -v, --validate  First filter with a DFA, then apply skip regex.
    -a, --append    Summarize the append events.
    -n, --named     Summarize the named events.
    -h, --help      Print this help message.
";

#[derive(Deserialize)]
struct Args {
    arg_log_file: String,

    flag_validate: bool,
    flag_append: bool,
    flag_named: bool,

    // docopt has a builtin help flag handler if you include it in your
    // usage message.
    // flag_help: bool,
}

macro_rules! regex {
    ($re:expr, $skip_validate:expr) => {{
        use regex::bytes::Regex;
        use regex::internal::ExecBuilder;

        if $skip_validate {
            ExecBuilder::new($re).skip_backtrack()
                .skip_validate(true)
                .only_utf8(false).build()
                .map(regex::bytes::Regex::from)
                .unwrap()
        } else {
            Regex::new($re).unwrap()
        }
    }}
}

struct Stats {
    no_lines: usize,
    appends: usize,
    max_append_offset: usize,
    min_append_offset: usize,
    total_bytes_written: usize,
    named_events: usize,
    named_hist: HashMap<String, usize>,
}
impl Stats {
    fn new() -> Self {
        Stats {
            no_lines: 0,
            appends: 0,
            max_append_offset: 0,
            min_append_offset: usize::max_value(),
            total_bytes_written: 0,
            named_events: 0,
            named_hist: HashMap::new(),
        }
    }
}

/// A little script to generate a histogram of messages published in each
/// second.
fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let append_line =
        r"^.* with first offset: ([0-9]+).*value=([0-9]+).*$";
    let named_line = r".* scheduled task '(.+?)'.*";

    let append_re = regex!(append_line, args.flag_validate);
    let named_re = regex!(named_line, args.flag_validate);

    let mut stats = Stats::new();

    let file = File::open(&args.arg_log_file).expect("cannot open log file");
    let file = BufReader::new(file);
    for line in file.lines().filter_map(|result| result.ok()) {
        stats.no_lines += 1;
        if args.flag_append {
            scrape_append(&append_re, line.as_bytes(), &mut stats);
        }
        if args.flag_named {
            scrape_named(&named_re, line.as_bytes(), &mut stats);
        }
    } 

    let no_lines = stats.no_lines;
    let p = |x| ((x as f64) / (no_lines as f64)) * 100.0;

    if args.flag_append {
        println!("{}/{} ({:.2}%) append events",
                    stats.appends, stats.no_lines, p(stats.appends));
        println!("{} min offset", stats.min_append_offset);
        println!("{} max offset", stats.max_append_offset);
        println!("{} total bytes written", stats.total_bytes_written);
    }

    if args.flag_named {
        println!("{}/{} ({:.2}%) named events",
            stats.named_events, stats.no_lines, p(stats.named_events));

        let mut hist = stats.named_hist.drain()
            .map(|(e, n)| (n, e)).collect::<Vec<_>>();
        hist.sort_by(|lhs, rhs| rhs.cmp(lhs));
        let v = hist.iter().take(10).collect::<Vec<_>>();
        for &(ref n, ref e) in v.into_iter() {
            println!("event {} happened {} times.", e, n);
        }
    }
}

fn scrape_append<'a>(re: &Regex, line: &'a [u8], stats: &mut Stats)
{
    match re.captures(line) {
        Some(caps) => {
            stats.appends += 1;
            let off = String::from_utf8_lossy(&caps[1])
                        .parse::<usize>().unwrap();
            stats.max_append_offset =
                cmp::max(stats.max_append_offset, off);
            stats.min_append_offset =
                cmp::min(stats.min_append_offset, off);

            let nbytes = String::from_utf8_lossy(&caps[2])
                        .parse::<usize>().unwrap();
            stats.total_bytes_written += nbytes;
        }
        None => (),
    }
}

fn scrape_named(re: &Regex, line: &[u8], stats: &mut Stats) {
    match re.captures(line) {
        Some(caps) => {
            stats.named_events += 1;
            *stats.named_hist
                .entry(String::from_utf8_lossy(&caps[1]).to_string())
                .or_insert(0) += 1;
        }
        None => (),
    }
}
