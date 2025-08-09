use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;
use memmap2::MmapOptions;
use memchr::memchr_iter;
use rustc_hash::FxHashMap;
use crate::{WeatherRecord, WeatherError, StationStats, HashTable};

// ============================================================================
// Stage 1: File Reading
// ============================================================================

pub fn read_file_raw_buffered(file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn read_file_raw_mmap(file_path: &str) -> Result<memmap2::Mmap, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };
    Ok(mmap)
}

// ============================================================================
// Stage 2: Line Splitting
// ============================================================================

pub fn split_into_lines_basic(data: &[u8]) -> Vec<&[u8]> {
    let mut lines = Vec::new();
    let mut start = 0;
    
    for (i, &byte) in data.iter().enumerate() {
        if byte == b'\n' {
            if start < i {
                lines.push(&data[start..i]);
            }
            start = i + 1;
        }
    }
    
    // Handle last line if it doesn't end with newline
    if start < data.len() {
        lines.push(&data[start..]);
    }
    
    lines
}

pub fn split_into_lines_simd(data: &[u8]) -> Vec<&[u8]> {
    let mut lines = Vec::new();
    let mut start = 0;
    
    for newline_pos in memchr_iter(b'\n', data) {
        if start < newline_pos {
            lines.push(&data[start..newline_pos]);
        }
        start = newline_pos + 1;
    }
    
    // Handle last line if it doesn't end with newline
    if start < data.len() {
        lines.push(&data[start..]);
    }
    
    lines
}

// ============================================================================
// Stage 3: Record Parsing
// ============================================================================

pub fn parse_records_string(lines: &[&[u8]]) -> Result<Vec<WeatherRecord>, WeatherError> {
    let mut records = Vec::with_capacity(lines.len());
    
    for (line_num, &line_bytes) in lines.iter().enumerate() {
        // Skip empty lines
        if line_bytes.is_empty() {
            continue;
        }
        
        let line = std::str::from_utf8(line_bytes)
            .map_err(|_| WeatherError::InvalidFormat(
                format!("Line {}: Invalid UTF-8 encoding", line_num + 1)
            ))?;
            
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() != 2 {
            return Err(WeatherError::InvalidFormat(
                format!("Line {} does not have exactly 2 columns separated by ';'. Found {} columns", 
                       line_num + 1, parts.len())
            ));
        }

        let station = parts[0].trim().to_string();
        if station.is_empty() {
            return Err(WeatherError::InvalidFormat(
                format!("Line {}: Weather station name cannot be empty", line_num + 1)
            ));
        }

        let temperature_str = parts[1].trim();
        let temperature = f64::from_str(temperature_str)
            .map_err(|_| WeatherError::Parse(
                format!("Line {}: Cannot parse temperature '{}' as a number", 
                       line_num + 1, temperature_str)
            ))?;

        records.push(WeatherRecord::new(station, temperature));
    }
    
    Ok(records)
}

pub fn parse_records_bytes(lines: &[&[u8]]) -> Result<Vec<WeatherRecord>, WeatherError> {
    let mut records = Vec::with_capacity(lines.len());
    
    for (line_num, &line_bytes) in lines.iter().enumerate() {
        // Skip empty lines
        if line_bytes.is_empty() {
            continue;
        }
        
        // Find semicolon position
        let semicolon_pos = memchr::memchr(b';', line_bytes)
            .ok_or_else(|| WeatherError::InvalidFormat(
                format!("Line {}: No semicolon delimiter found", line_num + 1)
            ))?;
            
        if semicolon_pos == 0 {
            return Err(WeatherError::InvalidFormat(
                format!("Line {}: Weather station name cannot be empty", line_num + 1)
            ));
        }
        
        // Extract station name (trim whitespace)
        let station_bytes = &line_bytes[..semicolon_pos];
        let station_str = std::str::from_utf8(station_bytes)
            .map_err(|_| WeatherError::InvalidFormat(
                format!("Line {}: Invalid UTF-8 in station name", line_num + 1)
            ))?;
        let station = station_str.trim().to_string();
        
        // Extract temperature (trim whitespace)
        let temp_bytes = &line_bytes[semicolon_pos + 1..];
        let temp_str = std::str::from_utf8(temp_bytes)
            .map_err(|_| WeatherError::InvalidFormat(
                format!("Line {}: Invalid UTF-8 in temperature", line_num + 1)
            ))?;
        let temperature_str = temp_str.trim();
        let temperature = f64::from_str(temperature_str)
            .map_err(|_| WeatherError::Parse(
                format!("Line {}: Cannot parse temperature '{}' as a number", 
                       line_num + 1, temperature_str)
            ))?;

        records.push(WeatherRecord::new(station, temperature));
    }
    
    Ok(records)
}

pub fn parse_records_unsafe(lines: &[&[u8]]) -> Result<Vec<WeatherRecord>, WeatherError> {
    let mut records = Vec::with_capacity(lines.len());
    
    for (line_num, &line_bytes) in lines.iter().enumerate() {
        // Skip empty lines
        if line_bytes.is_empty() {
            continue;
        }
        
        // Find semicolon position
        let semicolon_pos = memchr::memchr(b';', line_bytes)
            .ok_or_else(|| WeatherError::InvalidFormat(
                format!("Line {}: No semicolon delimiter found", line_num + 1)
            ))?;
            
        if semicolon_pos == 0 {
            return Err(WeatherError::InvalidFormat(
                format!("Line {}: Weather station name cannot be empty", line_num + 1)
            ));
        }
        
        // Extract station name (unsafe UTF-8 conversion - assumes valid UTF-8)
        let station_bytes = &line_bytes[..semicolon_pos];
        let station_str = unsafe { std::str::from_utf8_unchecked(station_bytes) };
        let station = station_str.trim().to_string();
        
        // Extract temperature (unsafe UTF-8 conversion)
        let temp_bytes = &line_bytes[semicolon_pos + 1..];
        let temp_str = unsafe { std::str::from_utf8_unchecked(temp_bytes) };
        let temperature_str = temp_str.trim();
        let temperature = f64::from_str(temperature_str)
            .map_err(|_| WeatherError::Parse(
                format!("Line {}: Cannot parse temperature '{}' as a number", 
                       line_num + 1, temperature_str)
            ))?;

        records.push(WeatherRecord::new(station, temperature));
    }
    
    Ok(records)
}

// ============================================================================
// Stage 4: Data Aggregation
// ============================================================================

pub fn aggregate_records_std(records: &[WeatherRecord]) -> HashTable<String, StationStats> {
    let mut station_stats: HashTable<String, StationStats> = HashTable::new();
    
    for record in records {
        match station_stats.get(&record.station) {
            Some(existing_stats) => {
                let mut updated_stats = existing_stats.clone();
                updated_stats.add_temperature(record.temperature);
                station_stats.insert(record.station.clone(), updated_stats);
            }
            None => {
                let new_stats = StationStats::new(record.station.clone(), record.temperature);
                station_stats.insert(record.station.clone(), new_stats);
            }
        }
    }
    
    station_stats
}

pub fn aggregate_records_fx(records: &[WeatherRecord]) -> FxHashMap<String, StationStats> {
    let mut station_stats: FxHashMap<String, StationStats> = FxHashMap::default();
    
    for record in records {
        match station_stats.get_mut(&record.station) {
            Some(stats) => {
                stats.add_temperature(record.temperature);
            }
            None => {
                let new_stats = StationStats::new(record.station.clone(), record.temperature);
                station_stats.insert(record.station.clone(), new_stats);
            }
        }
    }
    
    station_stats
}

pub fn aggregate_records_streaming<I>(records: I) -> HashTable<String, StationStats> 
where 
    I: Iterator<Item = WeatherRecord>
{
    let mut station_stats: HashTable<String, StationStats> = HashTable::new();
    
    for record in records {
        match station_stats.get(&record.station) {
            Some(existing_stats) => {
                let mut updated_stats = existing_stats.clone();
                updated_stats.add_temperature(record.temperature);
                station_stats.insert(record.station.clone(), updated_stats);
            }
            None => {
                let new_stats = StationStats::new(record.station.clone(), record.temperature);
                station_stats.insert(record.station.clone(), new_stats);
            }
        }
    }
    
    station_stats
}

// ============================================================================
// Stage 5: Full Pipeline Variants
// ============================================================================

pub fn pipeline_current(file_path: &str) -> Result<HashTable<String, StationStats>, Box<dyn std::error::Error>> {
    // Use existing implementation
    crate::process_weather_file_silent(file_path)
}

pub fn pipeline_mmap_string(file_path: &str) -> Result<HashTable<String, StationStats>, Box<dyn std::error::Error>> {
    let mmap = read_file_raw_mmap(file_path)?;
    let lines = split_into_lines_simd(&mmap);
    let records = parse_records_string(&lines)?;
    Ok(aggregate_records_std(&records))
}

pub fn pipeline_mmap_bytes(file_path: &str) -> Result<HashTable<String, StationStats>, Box<dyn std::error::Error>> {
    let mmap = read_file_raw_mmap(file_path)?;
    let lines = split_into_lines_simd(&mmap);
    let records = parse_records_bytes(&lines)?;
    Ok(aggregate_records_std(&records))
}

pub fn pipeline_mmap_unsafe(file_path: &str) -> Result<HashTable<String, StationStats>, Box<dyn std::error::Error>> {
    let mmap = read_file_raw_mmap(file_path)?;
    let lines = split_into_lines_simd(&mmap);
    let records = parse_records_unsafe(&lines)?;
    Ok(aggregate_records_std(&records))
}

pub fn pipeline_buffered_bytes(file_path: &str) -> Result<HashTable<String, StationStats>, Box<dyn std::error::Error>> {
    let data = read_file_raw_buffered(file_path)?;
    let lines = split_into_lines_simd(&data);
    let records = parse_records_bytes(&lines)?;
    Ok(aggregate_records_std(&records))
}

pub fn pipeline_streaming(file_path: &str) -> Result<HashTable<String, StationStats>, Box<dyn std::error::Error>> {
    // Streaming version that doesn't load everything into memory
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut station_stats: HashTable<String, StationStats> = HashTable::new();
    
    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        let line = line.trim();
        
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() != 2 {
            return Err(Box::new(WeatherError::InvalidFormat(
                format!("Line {} does not have exactly 2 columns", line_num + 1)
            )));
        }

        let station = parts[0].trim().to_string();
        let temperature = f64::from_str(parts[1].trim())?;
        
        let record = WeatherRecord::new(station, temperature);
        
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