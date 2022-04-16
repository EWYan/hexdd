#[macro_use]
extern crate clap;

use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, prelude::*};

const MAX_COUNT:usize = std::usize::MAX;

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
        None => MAX_COUNT
    };
    #[cfg(debug_assertions)] {
        println!("file_path :{}", file_path);
        println!("count :{}", bytes_cnt);
    }

    let mut fp = File::open(file_path)?;
    let mut buff = [0_u8; 32];
    let mut cnt_rec = 0_usize;
    loop {
        let cnt = fp.read(&mut buff)?;
        if cnt == 0 {
            break;
        }
        for b in &buff[..cnt] {
            print!("{:02x} ", b);
        }
        println!();
        cnt_rec += cnt;
        if cnt_rec > bytes_cnt {
            break;
        }
    }
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
