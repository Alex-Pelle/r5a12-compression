use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use bitstream_io::{BigEndian, BitWrite, LittleEndian};
use encode::encode_file;
use std::env;

mod entropie;
mod tree;
mod huffman;
mod encode;
mod utils;
mod decode;

use tree::Node;
use crate::decode::decode_file;
use crate::huffman::{max_encoded_length, number_of_symbols, to_list_for_canonical, to_ordered_list};
use crate::Mode::{DECODE, ENCODE};
use crate::utils::open_file;

enum Mode {
    ENCODE,
    DECODE
}

impl PartialEq for Mode {
    fn eq(&self, other: &Mode) -> bool {
        match self {
            ENCODE => *other == ENCODE,
            DECODE => *other == DECODE,
        }
    }
}

impl Eq for Mode {}

fn main() {

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let mut mode: Option<Mode> = None;
    let mut from: Option<String>= None;
    let mut to: Option<String> = None;

    for arg in &args[1..] {
        match arg.as_str() {
            "-c" => if mode == None {
                mode = Some(ENCODE);
            } else {
                println!("Un seul mode à la fois svp");
                return;
            },
            "-d" => if mode == None {
                mode = Some(DECODE);
            } else {
                println!("Un seul mode à la fois svp");
                return;
            },
            arg if arg.to_string().chars().next().unwrap() == '-' => {
                println!("Flag inconnu");
                return;
            }
            _ => {
                if from == None {
                    from = Some(arg.clone());
                }
                else if to == None {
                    to = Some(arg.clone());
                }
                else {
                    println!("Trop d'arguments, <mode> <input> <output>");
                    return;
                }
            }
        }
    }

    if mode == None || to == None {
        println!("Pas assez d'arguments, <mode> <input> <output>");
        return;
    }

    match mode.unwrap() {
        ENCODE => encode_file(from.unwrap(), to.unwrap()),
        DECODE => decode_file(from.unwrap(), to.unwrap())
    }

}







