use std::{
    fs,
    io::{BufRead, BufReader, Result, Write},
    path,
};

const COLOR_BLUE: &str = "\x1B[34m";
const COLOR_RED: &str = "\x1B[31m";
const COLOR_INITIAL: &str = "\x1B[0m";

#[derive(Default, Debug)]
pub struct CliOptions {
    ignore_case: bool,
}

impl CliOptions {
    pub fn default() -> Self {
        CliOptions {
            ..Default::default()
        }
    }
    pub fn new(args: &[String]) -> Self {
        let mut res = CliOptions::default();

        for arg in args {
            if arg == "--ignore-case" || arg == "-i" {
                res.ignore_case = true
            }
        }
        res
    }
    pub fn ignore_case(&mut self) -> &mut Self {
        self.ignore_case = true;
        self
    }
}

pub fn get_buffer_handler(cli_file_path: &str) -> Result<BufReader<fs::File>> {
    let file_path = path::PathBuf::from(cli_file_path);
    let file_handler = fs::File::open(file_path)?;
    Ok(BufReader::new(file_handler))
}

pub fn print_if_found(mut reader: BufReader<fs::File>, haystack: &str, opt: &CliOptions) {
    let mut dump = String::new();
    let mut current_line = 1;
    let haystack = if opt.ignore_case {
        haystack.to_ascii_lowercase()
    } else {
        haystack.to_owned()
    };
    let printer = |index, dump: &String, current_line| {
        print!(
            "{color}{line_no:>2} ",
            line_no = current_line,
            color = COLOR_BLUE
        );
        print!("{color}{txt}", color = COLOR_INITIAL, txt = &dump[..index]);
        print!(
            "{color}{txt}",
            txt = &dump[index..index + haystack.len()],
            color = COLOR_RED
        );
        print!(
            "{color}{txt}",
            color = COLOR_INITIAL,
            txt = &dump[index + haystack.len()..]
        );
        std::io::stdout().flush().unwrap();
    };
    while let Ok(s) = reader.read_line(&mut dump) {
        if s == 0 {
            break;
        } else if let Some(index) = dump.find(&haystack) {
            printer(index, &dump, current_line);
        } else if opt.ignore_case {
            if let Some(index) = dump
                .to_ascii_lowercase()
                .find(haystack.to_ascii_lowercase().as_str())
            {
                printer(index, &dump, current_line);
            }
        }
        current_line += 1;
        dump.clear()
    }
}

#[cfg(test)]
mod test_main_lib_internal {
    use super::*;

    #[test]
    fn test_buffer_provider() {
        let sample_file_path =
            env!("CARGO_MANIFEST_DIR").to_owned() + "/test/test_samples/short.txt";
        let mut handler = get_buffer_handler(&sample_file_path).unwrap();
        let mut res: Vec<u8> = Vec::new();
        handler.read_until(b'+', &mut res).unwrap();
        assert_eq!(
            String::from_utf8(res).unwrap(),
            include_str!("../test/../test/test_samples/short.txt")
        );
    }

    #[test]
    fn test_finding() {
        let sample_file_path =
            env!("CARGO_MANIFEST_DIR").to_owned() + "/test/test_samples/sample1.txt";
        let handler = get_buffer_handler(&sample_file_path).unwrap();
        print_if_found(handler, "this", &CliOptions::default());
    }
}
