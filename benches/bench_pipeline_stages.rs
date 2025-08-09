use criterion::{black_box, criterion_group, criterion_main, Criterion};
use obr::*;

const TEST_FILE: &str = "data/measurements_1KRecords.txt";

// ============================================================================
// Stage 1: File Reading Benchmarks
// ============================================================================

pub fn bench_file_reading(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_reading");
    
    group.bench_function("buffered_io", |b| {
        b.iter(|| {
            read_file_raw_buffered(black_box(TEST_FILE))
                .expect("Failed to read file")
        })
    });
    
    group.bench_function("mmap", |b| {
        b.iter(|| {
            read_file_raw_mmap(black_box(TEST_FILE))
                .expect("Failed to mmap file")
        })
    });
    
    group.finish();
}

// ============================================================================
// Stage 2: Line Splitting Benchmarks
// ============================================================================

pub fn bench_line_splitting(c: &mut Criterion) {
    let data = read_file_raw_buffered(TEST_FILE).expect("Failed to read test file");
    let mut group = c.benchmark_group("line_splitting");
    
    group.bench_function("basic_iterator", |b| {
        b.iter(|| {
            split_into_lines_basic(black_box(&data))
        })
    });
    
    group.bench_function("simd_memchr", |b| {
        b.iter(|| {
            split_into_lines_simd(black_box(&data))
        })
    });
    
    group.finish();
}

// ============================================================================
// Stage 3: Record Parsing Benchmarks
// ============================================================================

pub fn bench_parsing(c: &mut Criterion) {
    let data = read_file_raw_buffered(TEST_FILE).expect("Failed to read test file");
    let lines = split_into_lines_simd(&data);
    let mut group = c.benchmark_group("record_parsing");
    
    group.bench_function("string_based", |b| {
        b.iter(|| {
            parse_records_string(black_box(&lines))
                .expect("Failed to parse records")
        })
    });
    
    group.bench_function("byte_based", |b| {
        b.iter(|| {
            parse_records_bytes(black_box(&lines))
                .expect("Failed to parse records")
        })
    });
    
    group.bench_function("unsafe_parsing", |b| {
        b.iter(|| {
            parse_records_unsafe(black_box(&lines))
                .expect("Failed to parse records")
        })
    });
    
    group.finish();
}

// ============================================================================
// Stage 4: Aggregation Benchmarks
// ============================================================================

pub fn bench_aggregation(c: &mut Criterion) {
    let data = read_file_raw_buffered(TEST_FILE).expect("Failed to read test file");
    let lines = split_into_lines_simd(&data);
    let records = parse_records_bytes(&lines).expect("Failed to parse records");
    let mut group = c.benchmark_group("aggregation");
    
    group.bench_function("std_hashmap", |b| {
        b.iter(|| {
            aggregate_records_std(black_box(&records))
        })
    });
    
    group.bench_function("fx_hashmap", |b| {
        b.iter(|| {
            aggregate_records_fx(black_box(&records))
        })
    });
    
    group.bench_function("streaming", |b| {
        b.iter(|| {
            aggregate_records_streaming(black_box(records.iter().cloned()))
        })
    });
    
    group.finish();
}

// ============================================================================
// Stage 5: Full Pipeline Benchmarks
// ============================================================================

pub fn bench_full_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_pipeline");
    
    group.bench_function("current_impl", |b| {
        b.iter(|| {
            pipeline_current(black_box(TEST_FILE))
                .expect("Failed to process file")
        })
    });
    
    group.bench_function("mmap_string", |b| {
        b.iter(|| {
            pipeline_mmap_string(black_box(TEST_FILE))
                .expect("Failed to process file")
        })
    });
    
    group.bench_function("mmap_bytes", |b| {
        b.iter(|| {
            pipeline_mmap_bytes(black_box(TEST_FILE))
                .expect("Failed to process file")
        })
    });
    
    group.bench_function("mmap_unsafe", |b| {
        b.iter(|| {
            pipeline_mmap_unsafe(black_box(TEST_FILE))
                .expect("Failed to process file")
        })
    });
    
    group.bench_function("buffered_bytes", |b| {
        b.iter(|| {
            pipeline_buffered_bytes(black_box(TEST_FILE))
                .expect("Failed to process file")
        })
    });
    
    group.bench_function("streaming", |b| {
        b.iter(|| {
            pipeline_streaming(black_box(TEST_FILE))
                .expect("Failed to process file")
        })
    });
    
    group.finish();
}

// ============================================================================
// Individual Stage Timing (for identifying bottlenecks)
// ============================================================================

pub fn bench_stage_timing(c: &mut Criterion) {
    let mut group = c.benchmark_group("stage_timing");
    
    // Time just the file reading
    group.bench_function("01_file_read_mmap", |b| {
        b.iter(|| {
            let _mmap = read_file_raw_mmap(black_box(TEST_FILE))
                .expect("Failed to mmap file");
        })
    });
    
    // Time file read + line splitting
    group.bench_function("02_read_and_split", |b| {
        b.iter(|| {
            let mmap = read_file_raw_mmap(black_box(TEST_FILE))
                .expect("Failed to mmap file");
            let _lines = split_into_lines_simd(&mmap);
        })
    });
    
    // Time file read + line splitting + parsing
    group.bench_function("03_read_split_parse", |b| {
        b.iter(|| {
            let mmap = read_file_raw_mmap(black_box(TEST_FILE))
                .expect("Failed to mmap file");
            let lines = split_into_lines_simd(&mmap);
            let _records = parse_records_bytes(&lines)
                .expect("Failed to parse records");
        })
    });
    
    // Time complete pipeline
    group.bench_function("04_complete_pipeline", |b| {
        b.iter(|| {
            let mmap = read_file_raw_mmap(black_box(TEST_FILE))
                .expect("Failed to mmap file");
            let lines = split_into_lines_simd(&mmap);
            let records = parse_records_bytes(&lines)
                .expect("Failed to parse records");
            let _stats = aggregate_records_std(&records);
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_file_reading,
    bench_line_splitting,
    bench_parsing,
    bench_aggregation,
    bench_full_pipeline,
    bench_stage_timing
);
criterion_main!(benches);