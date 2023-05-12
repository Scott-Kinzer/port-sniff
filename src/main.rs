use::std::env;
use std::process;
use port_sniffer::Arguments;
use std::thread;
use std::sync::mpsc::channel;
mod scan;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut program = args.clone();

    program.remove(0);

    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{:?}, problem parsing args: {}", program, err);
            process::exit(0);
        }
    });

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;

    let (tx, rx) = channel();

    for i in 0..num_threads {
        let tx  = tx.clone();
        thread::spawn(move || {
            scan::scan_mod::scan_port(tx, i, addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);

    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();

    for i in out {
        println!("{}", i);
    }

}
