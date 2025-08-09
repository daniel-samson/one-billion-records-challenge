use std::env;
use std::process;
use obr::read_weather_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <weather_file.csv>", args[0]);
        eprintln!("Example: {} weather_data.csv", args[0]);
        process::exit(1);
    }
    
    let file_path = &args[1];
    
    match read_weather_file(file_path) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
