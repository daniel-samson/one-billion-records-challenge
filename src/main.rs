use std::env;
use std::process;
use obr::{WeatherCsvReader, HashTable, StationStats};

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

fn read_weather_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = WeatherCsvReader::from_path(file_path)?;
    let mut station_stats: HashTable<String, StationStats> = HashTable::new();
    
    for record_result in reader.records() {
        let record = record_result?;
        
        match station_stats.get(&record.station) {
            Some(existing_stats) => {
                let mut updated_stats = existing_stats.clone();
                updated_stats.add_temperature(record.temperature);
                station_stats.insert(record.station, updated_stats);
            }
            None => {
                let new_stats = StationStats::new(record.station.clone(), record.temperature);
                station_stats.insert(record.station, new_stats);
            }
        }
    }
    
    if station_stats.is_empty() {
        eprintln!("No weather records found in the file.");
        return Ok(());
    }
    
    // Output CSV headers
    println!("Station,Records,MinTemperature,MaxTemperature,AvgTemperature");
    
    // Output each station's statistics in CSV format
    for (station, stats) in station_stats.iter() {
        println!("{},{},{:.1},{:.1},{:.1}",
                 station,
                 stats.count,
                 stats.min_temperature,
                 stats.max_temperature,
                 stats.avg_temperature());
    }
    
    Ok(())
}
