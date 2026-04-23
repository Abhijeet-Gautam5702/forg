# Welcome to 🦀 forg

**forg** is a high-performance, regex-powered file organization CLI tool built in Rust. It automates the tedious task of sorting files into designated directories based on custom pattern-matching rules, ensuring your workspace remains clean and structured.

### Why forg?

In a world where digital clutter accumulates in seconds, **forg** offers a surgical approach to file management. Whether you're a developer with a messy Downloads folder or a photographer organizing thousands of assets, **forg** provides the speed and flexibility you need.

### Core Philosophy

*   **Extreme Performance**: Engineered to be metadata-bound, achieving logical throughputs of over 10 GB/s in tests.
*   **Safety First**: Built-in dry-run modes and robust conflict resolution to protect your data.
*   **Regex Power**: Leverage the full power of Rust's regex engine for complex matching rules.
*   **Simple Configuration**: A clean JSON-based configuration that anyone can understand and modify.

{TIP type="admonition" title="Quick Start"}
Ready to clean up your workspace? Head over to the [Installation Guide](../getting-started/installation.md) and get started in seconds!
{/TIP}

### Key Features at a Glance

1.  **Priority-Based Sorting**: Rules are processed top-to-bottom; the first match always wins.
2.  **Conflict Resolution**: Choose between skipping, replacing (with backups), or auto-versioning conflicting files.
3.  **On-the-fly Mode**: Bypass your config for quick, one-off organization tasks.
4.  **Detailed Execution Reports**: Get a full breakdown of what happened, how fast it was, and any errors encountered.
