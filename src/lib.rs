use::std::net::IpAddr;
use std::str::FromStr;

pub struct Arguments {
    pub flag: String,
    pub ipaddr: IpAddr,
    pub threads: u16,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too much arguments");
        }

        let f = args[1].clone();

        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments { flag: String::from(""), ipaddr, threads: 4 });
        } else {

                    let f = args[1].clone();

                    if f.contains("-h") || f.contains("-help") && args.len() == 2 {
                        println!("Usage: -j to select how many threads you want \r\n -h or -help to show this help message");

                        return Err("Help")
                    } else if f.contains("h") || f.contains("help") {
                        return Err("Too many args");
                    } else if f.contains("-j") {
                        let ipaddr = match IpAddr::from_str(&args[3]) {
                            Ok(s) => s,
                            Err(_) => return Err("Not valid ip addr")
                        };

                        let threads = match args[2].parse::<u16>() {
                            Ok(s) => s,
                            Err(_) => return Err("failed to parse number")
                        };

                        return Ok(Arguments { flag: f, ipaddr, threads })
                    } else {
                       return Err("Invalid sntx");
                    }

        }

        
    }
}   
