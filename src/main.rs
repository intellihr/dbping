use url::Url;

use clap::{App, Arg};

use indicatif::ProgressBar;
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, SystemTime};

fn check_url(url: Url, timeout: Duration) -> bool {
    let mut result = false;
    let start_time = SystemTime::now();
    while start_time.elapsed().unwrap() < timeout {
        if let Ok(_) = TcpStream::connect(format!(
            "{}:{}",
            url.host_str().unwrap(),
            url.port().unwrap()
        )) {
            result = true;
            break;
        }

        thread::sleep(Duration::from_secs(1));
    }
    return result;
}

fn check_urls(urls: Vec<Url>, timeout: Duration) -> bool {
    let mut threads = vec![];
    for url in urls {
        threads.push(thread::spawn(move || -> bool {
            return check_url(url, timeout);
        }));
    }

    let (tx, rx) = mpsc::channel();
    let pb = ProgressBar::new(100);
    let pb_thread = thread::spawn(move || loop {
        if rx.try_recv().is_ok() {
            pb.finish_and_clear();
            break;
        }
        pb.inc(100 / (timeout.as_secs() + 1));
        thread::sleep(Duration::from_secs(1));
    });

    let mut results = vec![];
    for checker_thread in threads {
        results.push(checker_thread.join().unwrap());
    }
    let _ = tx.send(());
    pb_thread.join().unwrap();

    return results.iter().all(|&result| result);
}

fn main() -> Result<(), String> {
    let matches = App::new("dbwait")
        .version("0.0.1")
        .about("wait until db become available")
        .arg(
            Arg::with_name("timeout")
                .help("wait timeout in seconds")
                .short("t")
                .long("timeout")
                .takes_value(true)
                .default_value("10"),
        )
        .arg(
            Arg::with_name("checks")
                .help("db urls to check, e.g. postgresql://localhost:5432")
                .required(true)
                .index(1)
                .multiple(true),
        )
        .get_matches();

    let timeout = Duration::from_secs(matches.value_of("timeout").unwrap().parse::<u64>().unwrap());

    let mut result = Ok(());
    if let Some(checks) = matches.values_of("checks") {
        let target_urls: Result<Vec<_>, _> = checks.map(|check| Url::parse(check)).collect();
        match target_urls {
            Ok(urls) => {
                if !check_urls(urls, timeout) {
                    result = Err("timeout connecting to specified dbs".to_string());
                }
            }
            Err(e) => {
                result = Err(format!("Invalid URL provided: {:?}", e));
            }
        };
    }

    return result;
}
