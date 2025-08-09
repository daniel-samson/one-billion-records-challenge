use criterion::{black_box, criterion_group, criterion_main, Criterion};
use obr::{process_weather_file_silent, process_weather_file_silent_mmap};

pub fn bench_1k_records_regular(c: &mut Criterion) {
    let file_path = "data/measurements_1KRecords.txt";
    
    c.bench_function("1k_records_regular_io", |b| {
        b.iter(|| {
            process_weather_file_silent(black_box(file_path)).expect("Failed to process file")
        })
    });
}

pub fn bench_1k_records_mmap(c: &mut Criterion) {
    let file_path = "data/measurements_1KRecords.txt";
    
    c.bench_function("1k_records_mmap_io", |b| {
        b.iter(|| {
            process_weather_file_silent_mmap(black_box(file_path)).expect("Failed to process file")
        })
    });
}

criterion_group!(benches, bench_1k_records_regular, bench_1k_records_mmap);
criterion_main!(benches);
