use std::{
    env, 
    io::{
        self, 
        Write
    },
    net::{
        IpAddr, 
        Ipv4Addr
    }, 
    process, 
    sync::mpsc::{
        channel, 
        Sender
    }, 
    thread
};

use bpaf::Bpaf;
use tokio::{net::TcpStream, task};

const MAX: u16 = 65535;

const IPFALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Arguments {
    #[bpaf(long, short, fallback(IPFALLBACK))]
    /// The address you want to sniff. Must be a valid IPv4 address. Falls back to 127.0.0.1
    pub address: IpAddr,
    #[bpaf(
        long("start"), 
        short('s'), 
        fallback(1u16), 
        guard(start_port_guard, "Must be greater than 0")
    )]
    /// The start port for the sniffer (must be greater than 0)
    pub start_port: u16,
    #[bpaf(
        long("end"), 
        short('e'), 
        fallback(MAX), 
        guard(end_port_guard, "Must be less than 65535")
    )]
    /// The end port for the sniffer (must be less than or equal to 65535)
    pub end_port: u16,
}

fn start_port_guard(input: &u16) -> bool {
    *input > 0
}

fn end_port_guard(input: &u16) -> bool {
    *input <= MAX
}

async fn scan(tx: Sender<u16>, port: u16, addr: IpAddr) {
    match TcpStream::connect(format!("{}:{}", addr, port)).await {
        Ok(_) => {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }
        Err(_) => {}
    }
}

#[tokio::main]
async fn main() {
    let opts = arguments().run();

    let (tx, rx) = channel();
    for i in opts.start_port..opts.end_port {
        let tx = tx.clone();
        task::spawn(async move {
            scan(tx, i, opts.address).await;   
        });
    }

    let mut out = vec![];
    drop(tx);

    for p in rx {
        out.push(p);
    }

    println!("");

    out.sort();
    for v in out {
        println!("{} is open",v);
    }
}   
