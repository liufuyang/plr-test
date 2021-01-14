use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use plr::regression::GreedyPLR;
use serde_json;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "plr", about = "A Greedy Piecewise Logistic Regression Tool")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    input_file: PathBuf,

    #[structopt(short, long, default_value = "8.0")]
    error_bound: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let mut plr = GreedyPLR::new(opt.error_bound); // gamma = 0.0005, the maximum regression error
    let mut segments = Vec::new();

    let file = File::open(opt.input_file)?;
    BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(n, line)| {
            let vec = line
                .unwrap_or_else(|e| panic!(format!("{} on line {}", e.to_string(), n + 1)))
                .split(",")
                .map(|s| {
                    s.parse::<f64>().unwrap_or_else(|e| {
                        panic!(format!(
                            "{} on line {}, while parsing string \"{}\"",
                            e.to_string(),
                            n + 1,
                            s
                        ))
                    })
                })
                .collect::<Vec<_>>();
            (n, vec)
        })
        .map(|(n, mut v)| {
            (
                v.pop().unwrap_or_else(|| {
                    panic!(format!(
                        "need 2 values but less than that on line {}, while parsing \"{:?}\"",
                        n + 1,
                        v
                    ))
                }),
                v.pop().unwrap_or_else(|| {
                    panic!(format!(
                        "need 2 values but less than that on line {}, while parsing \"{:?}\"",
                        n + 1,
                        v
                    ))
                }),
            )
        })
        .for_each(|(v, k)| {
            if let Some(segment) = plr.process(k, v) {
                segments.push(segment);
            }
        });
    // because we have a finite amount of data, we flush the buffer and get the potential
    // last segment.
    if let Some(segment) = plr.finish() {
        segments.push(segment);
    }

    let segments_json = serde_json::to_string(&segments)?;
    println!("{}", segments_json);
    // Output will be something like:
    // [{"start":1.0,"stop":5.0,"slope":25.0,"intercept":-17.0},{"start":5.0,"stop":7.0,"slope":110.0,"intercept":-48.0},{"start":7.0,"stop":1.7976931348623157e308,"slope":38.0,"intercept":520.0}]

    Ok(())
}
