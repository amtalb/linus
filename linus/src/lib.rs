use std::fs;

pub struct Source {
    pub string: String,
}

impl Source {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Source, &'static str> {
        args.next();

        // the file path is the first argument
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("No source file"),
        };

        // open and read the file into a string
        let string = fs::read_to_string(file_path).expect("Could not read file");

        Ok(Source { string })
    }
}
