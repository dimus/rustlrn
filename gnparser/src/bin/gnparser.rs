use async_std::task;
use clap::crate_version;
// use crossbeam_channel::{bounded, Receiver, Sender};
use futures::channel::mpsc;
use futures::executor::ThreadPool;
use futures::stream::StreamExt;
use log::{error, info};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path;
use std::process;
use std::time::Instant;
use stderrlog::{self, Timestamp};

use gnparser::{GNParser, Method};

#[macro_use]
extern crate clap;

fn main() {
    stderrlog::new()
        .verbosity(2)
        .timestamp(Timestamp::Second)
        .init()
        .unwrap();
    use clap::App;
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("gnparser.yml");
    let mut app = App::from_yaml(yaml).version(crate_version!());
    let matches = app.clone().get_matches();
    let mut gnp = GNParser::new();
    if let Some(method_str) = matches.value_of("method") {
        match Method::new(method_str) {
            Ok(method) => {
                gnp.method(method);
            }
            Err(err) => {
                error!("using default method {}: {}", gnp.method.to_string(), err);
            }
        }
    }
    if let Some(ref input) = matches.value_of("INPUT") {
        if path::Path::new(input).exists() {
            let f = File::open(input).unwrap();
        // match parse_file(gnp, f) {
        //     Ok(_) => process::exit(0),
        //     Err(err) => {
        //         error!("{}", err);
        //         process::exit(1);
        //     }
        // }
        } else {
            task::block_on(gnp.parse(vec![input.to_string()])).unwrap();
        }
    } else if is_readable_stdin() {
        // match parse_file(gnp, io::stdin()) {
        //     Ok(_) => process::exit(0),
        //     Err(err) => {
        //         println!("{:#?}", err);
        //         process::exit(1);
        //     }
        // }
    } else {
        app.print_long_help().unwrap();
    };
}

// async fn parse_file<'a, R>(gnp: GNParser, r: R)
// where
//     R: Read,
// {
//     let pool = ThreadPool::new().expect("Failed to build pool");
//     let (in_s, in_r) = mpsc::unbounded();
//     let batch_size = gnp.batch_size;
//
//
//     let rdr = BufReader::new(r);
//     let fut_tx_result = prepare_inputs(rdr, in_s, batch_size);
//     pool.spawn_ok(fut_txt_result);
//     let fut_values = for r in in_r {
//         r.collect()
//     }
//     fut_values.await?
// }

// fn process_outputs(gnp: gnparser::GNParser, out_r: Receiver<Vec<String>>, done_s: Sender<bool>) {
//     let mut is_first = true;
//     for outputs in out_r {
//         gnp.format_outputs(outputs, is_first);
//         is_first = false;
//     }
//     done_s.send(true).unwrap();
// }

// fn prepare_inputs<R>(rdr: R, in_s: Sender<Vec<String>>, batch_size: usize)
// where
//     R: BufRead,
// {
//     let mut inputs: Vec<String> = Vec::with_capacity(batch_size);
//     let time_start = Instant::now();
//
//     for (i, l) in rdr.lines().enumerate() {
//         if inputs.len() == batch_size {
//             in_s.send(inputs).unwrap();
//             inputs = Vec::with_capacity(batch_size);
//         }
//         if (i + 1) % 10_000 == 0 {
//             let duration = time_start.elapsed().as_secs() as f32;
//             let speed = (i + 1) as f32 / duration;
//             info!("Processed {} rows, {:.0} names/sec", i + 1, speed);
//         }
//         if let Ok(line) = l {
//             inputs.push(line.trim().to_owned());
//         };
//     }
//     in_s.send(inputs).unwrap();
//     drop(in_s);
// }

/// Returns true if and only if stdin is believed to be readable.
///
/// When stdin is readable, command line programs may choose to behave
/// differently than when stdin is not readable. For example, `command foo`
/// might search the current directory for occurrences of `foo` where as
/// `command foo < some-file` or `cat some-file | command foo` might instead
/// only search stdin for occurrences of `foo`.
pub fn is_readable_stdin() -> bool {
    #[cfg(unix)]
    fn imp() -> bool {
        use same_file::Handle;
        use std::os::unix::fs::FileTypeExt;

        let ft = match Handle::stdin().and_then(|h| h.as_file().metadata()) {
            Err(_) => return false,
            Ok(md) => md.file_type(),
        };
        ft.is_file() || ft.is_fifo()
    }

    #[cfg(windows)]
    fn imp() -> bool {
        use winapi_util as winutil;

        winutil::file::typ(winutil::HandleRef::stdin())
            .map(|t| t.is_disk() || t.is_pipe())
            .unwrap_or(false)
    }

    !is_tty_stdin() && imp()
}

/// Returns true if and only if stdin is believed to be connectted to a tty
/// or a console.
pub fn is_tty_stdin() -> bool {
    atty::is(atty::Stream::Stdin)
}
