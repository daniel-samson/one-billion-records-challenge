use std::env;
use std::process;
use std::collections::HashSet;
use obr::WeatherCsvReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <weather_file.csv>", args[0]);
        eprintln!("Example: {} weather_data.csv", args[0]);
        process::exit(1);
    }
    
    let file_path = &args[1];
    
    match read_weather_stats(file_path) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn read_weather_stats(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = WeatherCsvReader::from_path(file_path)?;
    let mut unique_stations: HashSet<String> = HashSet::new();
    let mut longest_station_name_length = 0;
    let mut total_records = 0;
    let mut longest_station_name = String::new();
    
    for record_result in reader.records() {
        let record = record_result?;
        total_records += 1;
        
        // Track unique stations
        unique_stations.insert(record.station.clone());
        
        // Track longest station name
        if record.station.len() > longest_station_name_length {
            longest_station_name_length = record.station.len();
            longest_station_name = record.station.clone();
        }
    }
    
    if total_records == 0 {
        eprintln!("No weather records found in the file.");
        return Ok(());
    }
    
    // Output statistics in field: value format
    println!("TotalStations: {}", unique_stations.len());
    println!("LongestStationNameLength: {}", longest_station_name_length);
    println!("LongestStationName: {}", longest_station_name);
    println!("TotalRecords: {}", total_records);
    
    Ok(())
}