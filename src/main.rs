use find;

const HELP_MESSAGE: &str = "HELP: program_name file_path haystack";

fn main() {
    let argv: Vec<_> = std::env::args().collect();
    if argv.len() < 3 {
        eprintln!("{}", HELP_MESSAGE);
        std::process::exit(1);
    }

    let (file_name, haystack) = (&argv[1].as_str(), &argv[2].as_str());
    let opt = if argv.len() >= 3 {
        find::CliOptions::new(&argv[3..])
    } else {
        find::CliOptions::default()
    };

    let handler = match find::get_buffer_handler(file_name) {
        Ok(val) => val,
        Err(er) => {
            println!("Exit with error: {}", er);
            std::process::exit(1);
        }
    };

    find::print_if_found(handler, haystack, &opt);
}
