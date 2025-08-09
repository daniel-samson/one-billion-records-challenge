use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct WeatherRecord {
    pub station: String,
    pub temperature: f64,
}

impl WeatherRecord {
    pub fn new(station: String, temperature: f64) -> Self {
        Self { station, temperature }
    }
}

#[derive(Debug)]
pub enum WeatherError {
    Io(std::io::Error),
    Parse(String),
    InvalidFormat(String),
}

impl From<std::io::Error> for WeatherError {
    fn from(error: std::io::Error) -> Self {
        WeatherError::Io(error)
    }
}

impl std::fmt::Display for WeatherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeatherError::Io(err) => write!(f, "I/O error: {}", err),
            WeatherError::Parse(msg) => write!(f, "Parse error: {}", msg),
            WeatherError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
        }
    }
}

impl std::error::Error for WeatherError {}

pub struct WeatherCsvReader<R> {
    reader: BufReader<R>,
}

impl WeatherCsvReader<File> {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, WeatherError> {
        let file = File::open(path)?;
        Ok(Self::from_reader(file))
    }
}

impl<R: std::io::Read> WeatherCsvReader<R> {
    pub fn from_reader(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
        }
    }

    pub fn read_all(mut self) -> Result<Vec<WeatherRecord>, WeatherError> {
        let mut records = Vec::new();
        for record in self.records() {
            records.push(record?);
        }
        Ok(records)
    }

    pub fn records(&mut self) -> WeatherRecordIterator<'_, R> {
        WeatherRecordIterator::new(&mut self.reader)
    }
}

pub struct WeatherRecordIterator<'a, R> {
    reader: &'a mut BufReader<R>,
    line_number: usize,
}

impl<'a, R: std::io::Read> WeatherRecordIterator<'a, R> {
    fn new(reader: &'a mut BufReader<R>) -> Self {
        Self {
            reader,
            line_number: 0,
        }
    }

    fn parse_line(&self, line: &str) -> Result<WeatherRecord, WeatherError> {
        let line = line.trim();
        
        if line.is_empty() {
            return Err(WeatherError::InvalidFormat(
                format!("Line {} is empty", self.line_number)
            ));
        }

        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() != 2 {
            return Err(WeatherError::InvalidFormat(
                format!("Line {} does not have exactly 2 columns separated by ';'. Found {} columns", 
                       self.line_number, parts.len())
            ));
        }

        let station = parts[0].trim().to_string();
        if station.is_empty() {
            return Err(WeatherError::InvalidFormat(
                format!("Line {}: Weather station name cannot be empty", self.line_number)
            ));
        }

        let temperature_str = parts[1].trim();
        let temperature = f64::from_str(temperature_str)
            .map_err(|_| WeatherError::Parse(
                format!("Line {}: Cannot parse temperature '{}' as a number", 
                       self.line_number, temperature_str)
            ))?;

        Ok(WeatherRecord::new(station, temperature))
    }
}

impl<'a, R: std::io::Read> Iterator for WeatherRecordIterator<'a, R> {
    type Item = Result<WeatherRecord, WeatherError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        
        loop {
            line.clear();
            self.line_number += 1;
            
            match self.reader.read_line(&mut line) {
                Ok(0) => return None, // EOF
                Ok(_) => {
                    // Skip empty lines
                    if line.trim().is_empty() {
                        continue;
                    }
                    return Some(self.parse_line(&line));
                }
                Err(e) => return Some(Err(WeatherError::Io(e))),
            }
        }
    }
}

#[derive(Debug)]
pub struct WeatherStats {
    pub total_records: usize,
    pub unique_stations: usize,
    pub min_temperature: f64,
    pub max_temperature: f64,
    pub avg_temperature: f64,
}

impl WeatherStats {
    pub fn from_records(records: &[WeatherRecord]) -> Option<Self> {
        if records.is_empty() {
            return None;
        }

        let mut stations = std::collections::HashSet::new();
        let mut sum = 0.0;
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        for record in records {
            stations.insert(&record.station);
            sum += record.temperature;
            min = min.min(record.temperature);
            max = max.max(record.temperature);
        }

        Some(Self {
            total_records: records.len(),
            unique_stations: stations.len(),
            min_temperature: min,
            max_temperature: max,
            avg_temperature: sum / records.len() as f64,
        })
    }
}

#[derive(Debug, Clone)]
pub struct StationStats {
    pub station_name: String,
    pub count: usize,
    pub min_temperature: f64,
    pub max_temperature: f64,
    pub sum_temperature: f64,
}

impl StationStats {
    pub fn new(station_name: String, temperature: f64) -> Self {
        Self {
            station_name,
            count: 1,
            min_temperature: temperature,
            max_temperature: temperature,
            sum_temperature: temperature,
        }
    }

    pub fn add_temperature(&mut self, temperature: f64) {
        self.count += 1;
        self.min_temperature = self.min_temperature.min(temperature);
        self.max_temperature = self.max_temperature.max(temperature);
        self.sum_temperature += temperature;
    }

    pub fn avg_temperature(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.sum_temperature / self.count as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_weather_record_creation() {
        let record = WeatherRecord::new("Station1".to_string(), 25.5);
        assert_eq!(record.station, "Station1");
        assert_eq!(record.temperature, 25.5);
    }

    #[test]
    fn test_parse_valid_csv() {
        let csv_data = "Station1;25.5\nStation2;-10.2\nStation3;0.0";
        let cursor = Cursor::new(csv_data);
        let mut reader = WeatherCsvReader::from_reader(cursor);
        
        let records = reader.read_all().unwrap();
        assert_eq!(records.len(), 3);
        
        assert_eq!(records[0].station, "Station1");
        assert_eq!(records[0].temperature, 25.5);
        
        assert_eq!(records[1].station, "Station2");
        assert_eq!(records[1].temperature, -10.2);
        
        assert_eq!(records[2].station, "Station3");
        assert_eq!(records[2].temperature, 0.0);
    }

    #[test]
    fn test_parse_with_whitespace() {
        let csv_data = "  Station1  ;  25.5  \n  Station2  ;  -10.2  ";
        let cursor = Cursor::new(csv_data);
        let mut reader = WeatherCsvReader::from_reader(cursor);
        
        let records = reader.read_all().unwrap();
        assert_eq!(records.len(), 2);
        
        assert_eq!(records[0].station, "Station1");
        assert_eq!(records[0].temperature, 25.5);
    }

    #[test]
    fn test_parse_with_empty_lines() {
        let csv_data = "Station1;25.5\n\n\nStation2;-10.2\n\n";
        let cursor = Cursor::new(csv_data);
        let mut reader = WeatherCsvReader::from_reader(cursor);
        
        let records = reader.read_all().unwrap();
        assert_eq!(records.len(), 2);
        
        assert_eq!(records[0].station, "Station1");
        assert_eq!(records[1].station, "Station2");
    }

    #[test]
    fn test_invalid_format_missing_semicolon() {
        let csv_data = "Station1 25.5";
        let cursor = Cursor::new(csv_data);
        let mut reader = WeatherCsvReader::from_reader(cursor);
        
        let result = reader.read_all();
        assert!(result.is_err());
        
        match result.unwrap_err() {
            WeatherError::InvalidFormat(msg) => {
                assert!(msg.contains("does not have exactly 2 columns"));
            }
            _ => panic!("Expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_invalid_format_too_many_columns() {
        let csv_data = "Station1;25.5;extra";
        let cursor = Cursor::new(csv_data);
        let mut reader = WeatherCsvReader::from_reader(cursor);
        
        let result = reader.read_all();
        assert!(result.is_err());
        
        match result.unwrap_err() {
            WeatherError::InvalidFormat(msg) => {
                assert!(msg.contains("Found 3 columns"));
            }
            _ => panic!("Expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_invalid_temperature() {
        let csv_data = "Station1;not_a_number";
        let cursor = Cursor::new(csv_data);
        let mut reader = WeatherCsvReader::from_reader(cursor);
        
        let result = reader.read_all();
        assert!(result.is_err());
        
        match result.unwrap_err() {
            WeatherError::Parse(msg) => {
                assert!(msg.contains("Cannot parse temperature"));
            }
            _ => panic!("Expected Parse error"),
        }
    }

    #[test]
    fn test_empty_station_name() {
        let csv_data = ";25.5";
        let cursor = Cursor::new(csv_data);
        let mut reader = WeatherCsvReader::from_reader(cursor);
        
        let result = reader.read_all();
        assert!(result.is_err());
        
        match result.unwrap_err() {
            WeatherError::InvalidFormat(msg) => {
                assert!(msg.contains("Weather station name cannot be empty"));
            }
            _ => panic!("Expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_iterator() {
        let csv_data = "Station1;25.5\nStation2;-10.2";
        let cursor = Cursor::new(csv_data);
        let mut reader = WeatherCsvReader::from_reader(cursor);
        
        let mut count = 0;
        for record in reader.records() {
            let record = record.unwrap();
            count += 1;
            assert!(!record.station.is_empty());
        }
        assert_eq!(count, 2);
    }

    #[test]
    fn test_weather_stats() {
        let records = vec![
            WeatherRecord::new("Station1".to_string(), 25.0),
            WeatherRecord::new("Station2".to_string(), -5.0),
            WeatherRecord::new("Station1".to_string(), 30.0),
            WeatherRecord::new("Station3".to_string(), 0.0),
        ];
        
        let stats = WeatherStats::from_records(&records).unwrap();
        assert_eq!(stats.total_records, 4);
        assert_eq!(stats.unique_stations, 3);
        assert_eq!(stats.min_temperature, -5.0);
        assert_eq!(stats.max_temperature, 30.0);
        assert_eq!(stats.avg_temperature, 12.5);
    }

    #[test]
    fn test_weather_stats_empty() {
        let records = vec![];
        let stats = WeatherStats::from_records(&records);
        assert!(stats.is_none());
    }

    #[test]
    fn test_station_stats() {
        let mut stats = StationStats::new("Station1".to_string(), 25.0);
        assert_eq!(stats.count, 1);
        assert_eq!(stats.avg_temperature(), 25.0);
        assert_eq!(stats.min_temperature, 25.0);
        assert_eq!(stats.max_temperature, 25.0);

        stats.add_temperature(15.0);
        assert_eq!(stats.count, 2);
        assert_eq!(stats.avg_temperature(), 20.0);
        assert_eq!(stats.min_temperature, 15.0);
        assert_eq!(stats.max_temperature, 25.0);

        stats.add_temperature(35.0);
        assert_eq!(stats.count, 3);
        assert_eq!(stats.avg_temperature(), 25.0);
        assert_eq!(stats.min_temperature, 15.0);
        assert_eq!(stats.max_temperature, 35.0);
    }
}