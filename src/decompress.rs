use crate::config::Config;
use crate::tree::{build_tree, Node, NodeType};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::{metadata, File};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

type DecompressCodeMap = BTreeMap<Vec<u8>, u8>;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let md = metadata(&config.inpfname)?;
    let inp_file_size = md.len();

    let inp_file = File::open(&config.inpfname)?;
    let mut buf_reader = BufReader::new(inp_file);

    let mut frequencies = [0usize; 256];

    let mut freq_bytes = [0; 8];
    for i in 0..256 {
        buf_reader.read(&mut freq_bytes)?;
        frequencies[i] = usize::from_le_bytes(freq_bytes);
    }

    let code_map = get_code_map(&frequencies);

    let out_file = File::create(&config.outfname)?;

    if inp_file_size == 2049 {
        println!("Decompressed successfully!");
        return Ok(());
    }

    let mut out_buf = BufWriter::new(out_file);

    let mut pos = 2049;
    let mut code: Vec<u8> = Vec::with_capacity(8);
    buf_reader.fill_buf()?;

    let mut last_byte: u8 = 0;
    let mut padding: u8 = 0;

    let mut is_end = false;

    loop {
        for byte in buf_reader.buffer().iter() {
            if pos == inp_file_size - 1 {
                last_byte = *byte;
            } else if pos == inp_file_size {
                padding = *byte;
                is_end = true;
            } else {
                for i in (0..8).rev() {
                    code.push((*byte >> i) & 1);
                    if code_map.contains_key(&code) {
                        let symbol = code_map.get(&code).unwrap();
                        out_buf.write(&[*symbol])?;
                        code.clear();
                    }
                }
            }
            pos += 1;
        }

        if is_end {
            for i in (0..(8 - padding)).rev() {
                code.push((last_byte >> i) & 1);
                if code_map.contains_key(&code) {
                    let symbol = code_map.get(&code).unwrap();
                    out_buf.write(&[*symbol])?;
                    code.clear();
                }
            }
            out_buf.flush()?;
        }

        buf_reader.consume(buf_reader.capacity());
        buf_reader.fill_buf()?;

        if buf_reader.buffer().is_empty() {
            out_buf.flush()?;
            break;
        }
    }

    println!("Decompressed successfully!");

    Ok(())
}

fn get_code_map(frequencies: &[usize; 256]) -> DecompressCodeMap {
    let mut code_map = DecompressCodeMap::new();
    let root = build_tree(frequencies);
    gen_codes(&root, Vec::new(), &mut code_map);
    code_map
}

fn gen_codes(node: &Node, prefix: Vec<u8>, codes: &mut DecompressCodeMap) {
    match node.node_type {
        NodeType::Internal(ref left_child, ref right_child) => {
            let mut left_prefix = prefix.clone();
            left_prefix.push(0);
            gen_codes(left_child, left_prefix, codes);

            let mut right_prefix = prefix;
            right_prefix.push(1);
            gen_codes(right_child, right_prefix, codes);
        }
        NodeType::Leaf(byte) => {
            codes.insert(prefix, byte);
        }
    }
}
