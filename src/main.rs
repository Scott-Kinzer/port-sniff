use::std::env;
use std::process;
use port_sniffer::Arguments;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut program = args.clone();

    program.remove(0);

    let _args = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{:?}, problem parsing args: {}", program, err);
            process::exit(0);
        }
    });
}
