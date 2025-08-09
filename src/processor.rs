use crate::{WeatherCsvReader, MmapWeatherCsvReader, HashTable, StationStats};

pub fn read_weather_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
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

#[inline]
pub fn process_weather_file_silent(file_path: &str) -> Result<HashTable<String, StationStats>, Box<dyn std::error::Error>> {
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
    
    Ok(station_stats)
}

pub fn read_weather_file_mmap(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = MmapWeatherCsvReader::from_path(file_path)?;
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

#[inline]
pub fn process_weather_file_silent_mmap(file_path: &str) -> Result<HashTable<String, StationStats>, Box<dyn std::error::Error>> {
    let mut reader = MmapWeatherCsvReader::from_path(file_path)?;
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
    
    Ok(station_stats)
}
