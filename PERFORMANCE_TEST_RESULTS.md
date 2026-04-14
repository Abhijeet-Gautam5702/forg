# Performance Details & Testing

This document provides a detailed breakdown of the performance tests conducted on **forg**. These tests were designed to measure how the tool handles varying file counts and data volumes.

## Test Environment
- **Operating System**: macOS (Darwin)
- **Filesystem**: APFS (Apple File System, optimized for metadata operations)
- **Disk Type**: NVMe SSD
---
## Summary Table

| Test ID | Total Files | Total Volume | Move Time | Avg. Throughput |
| :--- | :--- | :--- | :--- | :--- |
| **1A** | 5,000 | ~1 GB | 457 ms | **2,174 MB/s** |
| **1B** | 20,000 | ~3.8 GB | 1,885 ms | **2,077 MB/s** |
| **2A** | 5,000 | ~5.4 GB | 472 ms | **11,779 MB/s** |
| **2B** | 20,000 | ~22 GB | 2,023 ms | **11,271 MB/s** |

---

## Detailed Test Results

### TEST GROUP 1: Standard Office Environment
*Focus: High file count with moderate individual file sizes (100KB - 1MB).*

#### Test 1A (Small Batch, Small File Size)
- **Files**: 5,000
- **Composition**: 80% (100KB each), 15% (500KB each), 5% (1MB each)
- **Total Time**: 457ms *(Move Phase)* & 492ms *(Scanning + Grouping + Moving)*
- **Result**: Successfully moved 5,000 files in less than half a second.

#### Test 1B (Large Batch, Small File Size)
- **Files**: 20,000
- **Composition**: Same as 1A
- **Total Time**: 1,885ms *(Move Phase)* & 1,977ms *(Scanning + Grouping + Moving)*
- **Result**: Handled a massive 20k file directory in under 2 seconds.

---

### TEST GROUP 2: High Volume Environment
*Focus: Extreme data volumes and larger individual files (200KB - 10MB).*

#### Test 2A (High Volume Batch, Moderate File Size)
- **Files**: 5,000
- **Composition**: 80% (200KB each), 12.5% (2MB each), 7.5% (10MB each)
- **Total Volume**: 5.43 GB
- **Total Time**: 472 ms *(Move Phase)* & 505 ms *(Scanning + Grouping + Moving)*
- **Throughput**: **11.77 GB/s**

#### Test 2B (Extreme Batch)
- **Files**: 20,000
- **Composition**: Same as 2A
- **Total Volume**: 22.27 GB
- **Total Time**: 2,023 ms *(Move Phase)* & 2,127 ms *(Scanning + Grouping + Moving)*
- **Throughput**: **11.27 GB/s**

---

## Technical Insight: Why is it so fast?
The benchmarks reveal that `forg` is **metadata-bound**, not **I/O-bound**. Because `forg` utilizes atomic `fs::rename` operations, the time taken is proportional to the number of files (system calls), regardless of whether those files are 1KB or 10MB. 

This allows `forg` to achieve logical throughputs of over **10 GB/s**, which is faster than the sequential write speed of most modern NVMe SSDs.

---

## Reproducing Benchmarks

You can reproduce these results on your own machine using the provided `setup_performance_test_data.sh` script.

### 0. Create Test directories
1. Create a test source directory to store all the test-files:
    ```bash
    mkdir ~/test-dir
    ```
2. Create a test destination directory where all the test-files will be organised into:
    ```bash
    mkdir ~/test-dest-dir
    ```
3. If you already have a `config.json` present in `~/.forg/config.json`, replace it with the test config:
    ```bash
    > mv ~/.forg/config.json ~/.forg/config.json.bak
    > vim ~/.forg/config.json
    ```
    Copy this in `config.json`:
    ```json
    [
      {
        "pattern": ".*\\.(png|jpeg|jpg|gif|svg|webp|bmp|ico|tiff?)$",
        "path": "test-dest-dir/Pictures"
      },
      {
        "pattern": ".*\\.(mp4|mkv|mov|avi|wmv|flv|webm|m4v)$",
        "path": "test-dest-dir/Videos"
      },
      {
        "pattern": ".*\\.(mp3|wav|flac|m4a|ogg|aac)$",
        "path": "test-dest-dir/Music"
      },
      {
        "pattern": ".*\\.(pdf|txt|rtf|docx?|xlsx?|pptx?|odt|md|csv|epub|mobi)$",
        "path": "test-dest-dir/Documents"
      },
      {
        "pattern": ".*\\.(zip|tar|gz|rar|7z|bz2|xz)$",
        "path": "test-dest-dir/Archives"
      }
    ]
    ```

> *This is important so that you do not accidentally move the test-files into real directories of your system. It'll be a mess to delete the test files later!*

### 1. Preparation
1.  **Locate the Script**: Find `setup_performance_test_data.sh` in the `scripts/`.
2.  **Make it Executable**:
    ```bash
    > cd scripts
    > chmod +x setup_performance_test_data.sh
    ```

### 2. Configure the Test
Open `setup_performance_test_data.sh` and tweak the variables at the top to match the desired test group:
- `TOTAL_FILES`: Set to `5000` or `20000`.
- `CAT1_PERC`, `CAT2_PERC`, `CAT3_PERC`: Set the distribution percentages.
- `CAT1_SIZE`, `CAT2_SIZE`, `CAT3_SIZE`: Set the individual file sizes (e.g., `200k`, `2m`, `10m`).

### 3. Run the Generation
```bash
./setup_performance_test_data.sh
```

### 4. Run the Benchmark
Execute `forg` on the generated directory:
```bash
cargo run -- test-dir
```

---

## ⚠️ Important Considerations

> [!WARNING]
> **Storage Impact**: Running Test Group 2B will create **~22GB** of dummy data on your disk. Ensure you have sufficient free space before starting.

- **Cleanup**: After testing, remember to delete the test files to reclaim your disk space:
  ```bash
  rm -rf ~/test-dir
  rm -rf ~/test-dest-dir
  ```
- **Isolated Config**: It is highly recommended to use a clean configuration for benchmarking. You can temporarily move your existing config:
  ```bash
  mv ~/.forg/config.json ~/.forg/config.json.bak
  cargo run -- init
  ```
- **Filesystem Support**: These results were achieved on **APFS (Apple File System)**. Performance on other filesystems (like NTFS or ext4) may vary depending on how they handle rapid metadata updates and directory unlinking.
- **Hardware Variation**: Results will scale with your CPU's single-core performance and your SSD's controller latency.
