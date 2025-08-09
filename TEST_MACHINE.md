# Test Machine Specifications

This document describes the hardware and software configuration of the machine used for testing the One Billion Records Challenge implementation.

## Hardware Specifications

### CPU
- **Model**: Intel Core i7-11700K (11th Generation)
- **Base Clock**: 3.60 GHz
- **Max Turbo**: 5.00 GHz
- **Cores**: 8 physical cores
- **Threads**: 16 logical processors (Hyper-Threading)
- **Architecture**: x86_64
- **Cache**:
  - L1d: 384 KiB (8 instances)
  - L1i: 256 KiB (8 instances)
  - L2: 4 MiB (8 instances)
  - L3: 16 MiB (shared)
- **Features**: AVX-512, AES, SHA extensions

### Memory
- **Total RAM**: 64 GB (62 GiB)
- **Available**: ~53 GiB
- **Swap**: 4 GiB

### Storage
- **Primary Drive**: WDC WDS200T2B0C-00PXH0
- **Capacity**: 2 TB NVMe SSD
- **Available Space**: ~1.5 TB free
- **Interface**: NVMe (PCIe)

## Software Environment

### Operating System
- **Distribution**: Arch Linux
- **Kernel**: 6.15.8-arch1-2 
- **Architecture**: x86_64
- **Preemption Model**: PREEMPT_DYNAMIC

### Development Tools
- **Rust**: 1.89.0 (29483883e 2025-08-04)
- **Compiler**: rustc with LLVM backend
- **Target**: x86_64-unknown-linux-gnu

## Performance Characteristics

### CPU Features
- Modern Intel architecture with high single-thread and multi-thread performance
- 16 logical processors ideal for parallel processing
- Large L3 cache (16 MB) beneficial for data-intensive workloads
- Support for advanced instruction sets (AVX-512, AES) for optimized operations

### Memory Subsystem
- Large memory capacity (64 GB) allows processing of very large datasets in memory
- High-speed DDR4 memory with good bandwidth
- Minimal swap usage ensures consistent performance

### Storage
- NVMe SSD provides high I/O throughput for file operations
- Large capacity allows storage of multiple large test datasets
- Low latency storage ideal for sequential reads of large files

## Expected Performance Profile

This configuration should provide excellent performance for the One Billion Records Challenge:
- Sufficient memory to hold large datasets and intermediate results
- High-performance CPU with good single-thread performance for parsing
- Multi-core capability for potential parallel processing optimizations
- Fast storage for reading large input files efficiently

## Test Data Scale

With this hardware configuration, the implementation should comfortably handle:
- ✅ Sample datasets (KB range)
- ✅ 1K records (small test)
- ✅ 1M records (medium test)
- ✅ 1B records (full challenge) - with appropriate optimizations