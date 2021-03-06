#[macro_use]
extern crate clap;

use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, prelude::*};

const MAX_COUNT: usize = std::usize::MAX;

struct Printer {
    idx: usize,
}

impl Printer {
    fn new() -> Printer {
        Printer { idx: 1 }
    }
    fn print_byte(&mut self, b: u8) -> io::Result<()> {
        // print column header
        if self.idx == 1 {
            print!("offset\\col ");
            for ci in 0..16 {
                match ci {
                    7 => print!("{:02}  ", ci),
                    _ => print!("{:02} ", ci),
                }
            }
            println!();
        }
        // print offset 
        if self.idx % 16 == 1 {
            print!("0x{:08x} ", self.idx - 1);
        }
        // print byte
        print!("{:02x} ", b);
        // control LF
        match self.idx % 16 {
            8 => print!(" "),
            0 => {
                println!();
            },
            _ => {}
        }
        // work statistics
        self.idx += 1;
        Ok(())
    }
}
fn hexdump() -> io::Result<()> {
    let paras = Command::new(crate_name!())
        .arg(
            Arg::new("infile")
                .long("file")
                .short('f')
                .value_name("file path")
                .required(true)
                .help("specify a file"),
        )
        .arg(
            Arg::new("count")
                .long("count")
                .short('n')
                .takes_value(true)
                .help("specify print bytes counts"),
        )
        .get_matches();
    let file_path = paras.value_of("infile").unwrap();
    let bytes_cnt = match paras.value_of("count") {
        Some(cnt) => cnt.parse::<usize>().unwrap(),
        None => MAX_COUNT,
    };
    #[cfg(debug_assertions)]
    {
        println!("file_path :{}", file_path);
        println!("count :{}", bytes_cnt);
    }

    let mut fp = File::open(file_path)?;
    let mut buff = [0_u8; 32];
    let mut printer = Printer::new();
    'max_brk: loop {
        let cnt = fp.read(&mut buff)?;
        if cnt == 0 {
            break;
        }
        for b in &buff[..cnt] {
            let res = printer.print_byte(*b);
            match res {
                Ok(_) => {}, 
                Err(_) => break,
            }
            if printer.idx > bytes_cnt {
                break 'max_brk;
            }
        }
    }
    println!("\nTotal Bytes: 0x{:08X}({})", printer.idx - 1, printer.idx - 1);
    Ok(())
}

fn main() {
    let res = hexdump();
    match res {
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
        Ok(()) => {}
    }
}
