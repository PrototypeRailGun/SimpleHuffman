use crate::config::Config;
use crate::tree::{build_tree, Node, NodeType};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

type CompressCodeMap = Vec<Vec<u8>>;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let frequencies = get_frequencies(config)?;

    let out_file = File::create(&config.outfname)?;
    let mut out_buf = BufWriter::new(out_file);

    write_frequencies(&frequencies, &mut out_buf)?;

    let code_map = get_code_map(&frequencies);
    write_codes(config, &code_map, &mut out_buf)?;

    println!("Compressed successfully!");

    Ok(())
}

fn get_frequencies(config: &Config) -> Result<[usize; 256], Box<dyn Error>> {
    let mut frequencies = [1; 256];

    let inp_file = File::open(&config.inpfname)?;
    let mut buf_reader = BufReader::new(inp_file);
    buf_reader.fill_buf()?;

    for byte in buf_reader.buffer().iter() {
        frequencies[*byte as usize] += 1;
    }

    Ok(frequencies)
}

fn write_frequencies(
    frequencies: &[usize; 256],
    out_buf: &mut BufWriter<File>,
) -> Result<(), Box<dyn Error>> {
    for count in frequencies.iter() {
        out_buf.write(&count.to_le_bytes()[..])?;
    }
    Ok(())
}

fn get_code_map(frequencies: &[usize; 256]) -> CompressCodeMap {
    let mut code_map: CompressCodeMap = vec![Vec::new(); 256];
    let root = build_tree(frequencies);
    gen_codes(&root, Vec::new(), &mut code_map);
    code_map
}

fn gen_codes(node: &Node, prefix: Vec<u8>, codes: &mut CompressCodeMap) {
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
            codes[byte as usize] = prefix;
        }
    }
}

fn write_codes(
    config: &Config,
    code_map: &CompressCodeMap,
    out_buf: &mut BufWriter<File>,
) -> Result<(), Box<dyn Error>> {
    let inp_file = File::open(&config.inpfname)?;
    let mut buf_reader = BufReader::new(inp_file);
    buf_reader.consume(buf_reader.capacity());
    buf_reader.fill_buf()?;

    let mut packed_codes: u8 = 0;
    let mut num_bits: u8 = 0;

    loop {
        for byte in buf_reader.buffer().iter() {
            for bit in code_map.get(*byte as usize).unwrap() {
                if num_bits == 8 {
                    out_buf.write(&[packed_codes])?;
                    packed_codes = 0;
                    num_bits = 0;
                }
                packed_codes += packed_codes + bit;
                num_bits += 1;
            }
        }

        buf_reader.consume(buf_reader.capacity());
        buf_reader.fill_buf()?;
        if buf_reader.buffer().is_empty() {
            // Since the last byte written to the file contains garbage padding bits,
            // we will write their number to the file
            if num_bits > 0 {
                out_buf.write(&[packed_codes])?;
            }
            out_buf.write(&[8 - num_bits])?;
            break;
        }
    }

    Ok(())
}
