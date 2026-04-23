# Supported Platforms

**forg** is designed to be cross-platform within the Unix ecosystem. Below are the explicitly supported Operating Systems and Architectures.

## Operating Systems

| OS | Supported | Notes |
| :--- | :--- | :--- |
| **macOS** | ☑ | Full support for modern macOS versions. |
| **Linux** | ☑ | Supported on most major distributions (Ubuntu, Fedora, Arch, etc.). |
| **Windows** | ⛌ | Native Windows support is not currently available. |

## Architectures

| Architecture | macOS | Linux |
| :--- | :--- | :--- |
| **x86_64** (Intel/AMD 64-bit) | ☑ | ☑ |
| **arm64** (Apple Silicon / AArch64) | ☑ | ⛌ |

{NOTE type="admonition" title="Binary Availability"}
The official installation script automatically detects your OS and architecture to pull the correct binary. If your specific combination is not listed above, you may still be able to [Build from Source](../../getting-started/installation/installation.md).
{/NOTE}
