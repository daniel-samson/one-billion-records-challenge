# One Billion Records Challenge

A Rust implementation featuring a custom hash table for processing weather station data efficiently.

## Overview

This project implements a high-performance weather data processor using a custom hash table implementation. It can process large CSV files containing weather measurements and provide statistical analysis.

## Features

- Custom hash table implementation with XXHash
- Weather CSV reader for parsing weather station data  
- Memory-mapped file I/O for high performance
- Modular pipeline architecture for optimization testing
- SIMD-optimized line splitting with memchr
- Multiple hash table implementations (std::HashMap, FxHashMap)
- Comprehensive benchmarking suite for performance analysis
- Temperature statistics calculation (min, max, average)
- Two binary executables for different analysis types

## Project Structure

```
src/
├── lib.rs              # Library entry point
├── hash_table.rs       # Custom hash table implementation
├── weather.rs          # Weather data structures and CSV readers
├── xxhash.rs          # XXHash implementation  
├── processor.rs        # High-level processing functions
├── pipeline.rs         # Modular pipeline stages for optimization
├── main.rs            # Main weather analyzer
└── bin/
    └── stats.rs       # Weather data statistics analyzer

benches/
└── bench_pipeline_stages.rs # Detailed pipeline stage benchmarks
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

## Benchmarking and Performance Analysis

This project includes comprehensive benchmarking to identify performance bottlenecks and optimize for the one billion records challenge.

### Running Benchmarks

#### Detailed Pipeline Analysis

```bash
# Benchmark individual pipeline stages
cargo bench
```

### Pipeline Stages

The processing pipeline is broken down into measurable stages:

#### Stage 1: File Reading

- `read_file_raw_buffered()` - Traditional buffered I/O
- `read_file_raw_mmap()` - Memory-mapped file access

#### Stage 2: Line Splitting  

- `split_into_lines_basic()` - Basic iterator approach
- `split_into_lines_simd()` - SIMD-optimized with memchr

#### Stage 3: Record Parsing

- `parse_records_string()` - String-based parsing with split()
- `parse_records_bytes()` - Direct byte parsing with UTF-8 validation
- `parse_records_unsafe()` - Unsafe byte parsing (no UTF-8 validation)

#### Stage 4: Data Aggregation

- `aggregate_records_std()` - Using custom HashTable
- `aggregate_records_fx()` - Using FxHashMap (faster hasher)
- `aggregate_records_streaming()` - Iterator-based aggregation

#### Stage 5: Complete Pipelines

- `pipeline_current()` - Original implementation
- `pipeline_mmap_string()` - mmap + string parsing
- `pipeline_mmap_bytes()` - mmap + byte parsing  
- `pipeline_mmap_unsafe()` - mmap + unsafe parsing
- `pipeline_buffered_bytes()` - buffered I/O + byte parsing
- `pipeline_streaming()` - Streaming line-by-line processing

### Benchmark Categories

1. **Individual Stage Performance** - Isolate bottlenecks
2. **Pipeline Comparisons** - End-to-end performance
3. **Stage Timing** - Cumulative timing to identify where time is spent
4. **Memory vs Speed Tradeoffs** - Different approaches to processing

### Performance Characteristics

The implementation is optimized for:

- **Large Files**: Memory-mapped I/O reduces system call overhead
- **Multi-core Systems**: Foundation for parallel processing  
- **Memory Efficiency**: Streaming options for memory-constrained environments
- **String Processing**: SIMD optimizations and minimal allocations

### Current Performance (1K records on Intel i7-11700K)

- Regular I/O: ~352 µs
- mmap I/O: ~356 µs

*Note: mmap shows overhead for small files but will outperform on larger datasets*

## Dependencies

- `memmap2` - Memory-mapped file I/O
- `memchr` - SIMD-optimized byte searching  
- `rustc-hash` - Faster hash function (FxHash)
- `criterion` - Benchmarking framework

