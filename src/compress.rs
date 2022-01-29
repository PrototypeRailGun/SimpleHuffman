use crate::config::Config;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let frequencies = get_frequencies(config)?;

    let out_file = File::create(&config.outfname)?;
    let mut out_buf = BufWriter::new(out_file);

    // Write frequencies in the output file
    write_frequencies(&frequencies, &mut out_buf)?;

    Ok(())
}

fn get_frequencies(config: &Config) -> Result<[usize; 256], Box<dyn Error>> {
    let mut frequencies = [0; 256];

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
