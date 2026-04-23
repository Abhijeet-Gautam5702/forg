# Performance

**forg** is built for speed. By leveraging Rust's ownership model and efficient standard library, it achieves performance levels that make organizing even the largest directories feel instantaneous.

## Metadata-Bound Operations

The secret to **forg**'s speed is that it performs **metadata-bound** operations rather than I/O-bound ones.
*   **What this means**: When moving a file on the same filesystem, **forg** only tells the OS to update the file's pointer (its entry in the directory table).
*   **The Result**: Moving a 10 GB movie file takes the exact same amount of time as moving a 1 KB text file—usually less than a millisecond.

## Benchmarks

In our performance tests, **forg** achieved logical throughputs of over **10 GB/s**.

| Task | Total Files | Total Volume | Move Time |
| :--- | :--- | :--- | :--- |
| **Small Batch** | 5,000 | ~1 GB | **0.45s** |
| **Large Batch** | 20,000 | ~20 GB | **2.02s** |

*Note: Benchmarks were performed on an Apple M1 Pro with an NVMe SSD.*

## Optimization Techniques

1.  **Regex Pre-compilation**: All patterns in your `config.json` are compiled once at startup, ensuring that matching thousands of files doesn't incur redundant overhead.
2.  **Lazy Directory Creation**: **forg** only attempts to create a destination directory if it has at least one file to move into it.
3.  **BTreeMap for Organization**: Matches are tracked in sorted maps to optimize the execution flow and report generation.

{TIP type="admonition" title="Pro Performance Tip"}
To get the most out of **forg**, ensure your destination folders are on the same physical drive/partition as your source folder to take advantage of instantaneous metadata moves.
{/TIP}
