# One Billion Records Challenge

A Rust implementation featuring a custom hash table for processing weather station data efficiently.

## Overview

This project implements a high-performance weather data processor using a custom hash table implementation. It can process large CSV files containing weather measurements and provide statistical analysis.

## Features

- Custom hash table implementation with XXHash
- Weather CSV reader for parsing weather station data
- Temperature statistics calculation (min, max, average)
- Two binary executables for different analysis types

## Project Structure

```
src/
├── lib.rs              # Library entry point
├── hash_table.rs       # Custom hash table implementation
├── weather.rs          # Weather data structures
├── xxhash.rs          # XXHash implementation
├── main.rs            # Main weather analyzer
└── bin/
    └── stats.rs       # Weather data statistics analyzer
```

## Usage

### Building

```bash
cargo build --release
```

### Weather Analysis

Process weather data and output station statistics:

```bash
cargo run --bin obr data/sample_weather.csv
```

Output format:
```
Station,Records,MinTemperature,MaxTemperature,AvgTemperature
StationA,150,12.5,35.2,23.8
StationB,200,8.1,41.3,24.5
```

### File Statistics

Get basic statistics about the weather data file:

```bash
cargo run --bin stats data/sample_weather.csv
```

Output format:
```
TotalStations: 25
LongestStationNameLength: 15
LongestStationName: SomeVeryLongName
TotalRecords: 1500
```

## Sample Data

The `data/` directory contains sample files:
- `sample_weather.csv` - Small test dataset
- `measurements_1KRecords.txt` - 1,000 records
- `measurements_1MRecords.txt` - 1,000,000 records
- `bad_weather.csv` - Test file for error handling

## Development

### Testing
```bash
cargo test
```

### Linting
```bash
cargo clippy
```

### Formatting
```bash
cargo fmt
```

### Type Checking
```bash
cargo check
```

## Performance

This implementation uses a custom hash table optimized for string keys with XXHash for efficient processing of large weather datasets, making it suitable for the one billion records challenge scenario.