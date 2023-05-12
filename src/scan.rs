pub mod scan_mod {
    use std::net::{IpAddr, TcpStream};
    use std::io::{self, Write};
    use std::sync::mpsc::{Sender};

    const MAX: u16 = 65535;

    pub fn scan_port(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
        let mut port = start_port + 1;
    
        loop {
            match TcpStream::connect((addr, port)) {
                Ok(_) => {
                    println!(".");
                    io::stdout().flush().unwrap();
                    tx.send(port).unwrap();
                },
                Err(_) => {},
            }
    
            if MAX - port <= num_threads {
                break;
            }
    
            port += num_threads;
        }
    }
}