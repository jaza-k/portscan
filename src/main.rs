use std::env;
use std::net::IpAddr;
use std::str::FromStr; // FromStr trait allows conversion of String to IpAddr type
use std::process; // manages the way program shuts down/terminates
use std::sync::mpsc::{Sender, channel};
use std::thread;

const MAX: u16 = 65535; // max port # that can be sniffed

struct Arguments { // struct to define & hold arguments' type
    flag: String,
    ipaddress: IpAddr,
    threads: u16,
}

impl Arguments { // implementation block to allow instantiation of Arguments struct
    // method 'new' takes in args (reference to vector) and returns the Arguments struct in its result  
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 { // minimum 2 arguments required
            return Err("ERROR - Not enough arguments");
        } 
        else if args.len() > 4 {
            return Err("ERROR - Too many arguments");
        }
        let first = args[1].clone(); // create variable to look at first index of vector
        // if-let binding to destruct IpAddr and return a result
        if let Ok(ipaddress) = IpAddr::from_str(&first) {
            return Ok(Arguments{flag: String::from(""), ipaddress, threads: 4});
        }
        else { // else if converting 'first' to IP address fails
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -j to select number of threads\r\n-h or -help to show help message");
                return Err("Help");
            }
            else if flag.contains("-h") || flag.contains("-help") {
                return Err("ERROR - Too many arguments");
            }
            else if flag.contains("-j") {
                // match on turning arguments[3] to an IP address & bind it to variable 'ipaddr'
                let ipaddress = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("ERROR - Not a valid IPADDR; must be IPv4 or IPv6")
                };
                let threads = match args[2].parse::<u16>() { // change string input to u16 using parse()
                    Ok(s) => s,
                    Err(_) => return Err("ERROR - Failed to parse thread number")
                };
                return Ok(Arguments{threads, flag, ipaddress});
            }
            else {
                return Err("ERROR - Invalid syntax");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port<u16>, addr: IpAddr, num_threads: u16) {

}

fn main() {
    let args: Vec<String> = env::args().collect(); // take all arguments passed and place them in a Vec
    let program = args[0].clone();
    // create variable 'arguments' & call unwrap_or_else method on it to handle an error
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0); // call process::exit() & pass code 0 to avoid panic
            }
            else {
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(0);
            }
        }
    );
    
    let num_threads = arguments.threads; // bind arguments.threads to variable 'num_threads'
    let (tx, rx) = channel(); // instantiate a channel, destruct the tuple which is returned

    for i in 0..num_threads { // iterate from 0 to number of threads
        let tx = tx.clone(); // bind 'tx' to a separate tx, ensure each thread has its own transmitter

        thread::spawn(move || {
            scan(tx, i, arguments.ipaddress, num_threads);
        });
    }
}
