#[derive(Debug)]
pub enum Mode {
    Compress,
    Decompress,
}

#[derive(Debug)]
pub struct Config {
    pub rootdir: String,
    pub mode: Mode,
    pub inpfname: String,
    pub outfname: String,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("you must specify the mode and the input file.");
        }

        let rootdir = args[0].clone();
        let mode_arg = args[1].clone();
        let mode;

        if mode_arg == "compress" {
            mode = Mode::Compress;
        } else if mode_arg == "decompress" {
            mode = Mode::Decompress;
        } else {
            return Err(
                "the first argument allows only two options: \"compress\" or \"decompress\".",
            );
        }

        let inpfname = args[2].clone();
        let outfname;

        if let Some(out) = args.get(3) {
            outfname = out.clone();
        } else {
            outfname = String::from("out.txt");
        }

        Ok(Config {
            rootdir,
            mode,
            inpfname,
            outfname,
        })
    }
}
