# Syskit (Systems Administration Toolkit)

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Asynchronous](https://img.shields.io/badge/runtime-Tokio-lightgrey.svg)](https://tokio.rs/)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)]()

`syskit` is a highly performant, modern command-line interface (CLI) Swiss Army Knife engineered for low-level systems administration, network diagnostics, and defensive cryptographic operations. Built entirely in Rust and backed by the asynchronous `tokio` runtime, it delivers near-zero latency execution patterns wrapped in a structured DevOps engine.

---

# Technical Architecture & Core Modules

The toolkit enforces strict behavioral isolation by decoupling argument parsing from backend infrastructure, dividing operations into three core subsystems:
```
           ┌──────────────────────────────┐
           │    syskit CLI Entrypoint     │
           └──────────────┬───────────────┘
                          │
     ┌────────────────────┼────────────────────┐
     ▼                    ▼                    ▼
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│  system engine  │  │  network engine │  │ security engine │
├─────────────────┤  ├─────────────────┤  ├─────────────────┤
│ • Process Audit │  │ • Async Pinging │  │ • AES-256-GCM   │
│ • Task Killers  │  │ • Port Sweepers │  │ • Crypt Hashing │
│ • Host Metrics  │  │ • Data Pipelines│  │ • Data Shredder │
└─────────────────┘  └─────────────────┘  └─────────────────┘

```
*   **System Engine (`system`):** Interfaces with host machine diagnostics to extract native kernel metrics, track runtime allocation bounds, manage threads, and gracefully terminate targeting process states.
*   **Network Engine (`network`):** Coordinates asynchronous, non-blocking I/O routines for target discovery, automated port scanner sequences, data acquisition pipelines, and host validation telemetry.
*   **Security Engine (`security`):** Drives safe memory operations including multipass data shredding routines, along with industrial-grade primitives for file hashing and authenticated symmetric encryption.

---

## Features

*   **Real-Time Diagnostics:** Instantly evaluates host kernel telemetry, instruction pipelines, and resource availability on initial runtime instantiation.
*   **Asynchronous Network Verification:** Executes port validation sweeps and fast remote downloads without blocking execution loops.
*   **Defensive Cryptography:** Protects data paths with military-grade AES-256-GCM symmetric block ciphers and computes rigorous data hashes.
*   **Anti-Forensic Storage Purging:** Implements iterative block-overwriting algorithms to bypass standard disk-recovery systems.

---

# Installation

## Method 1: Global Installation via Cargo (Recommended)
If you have the Rust toolchain installed locally, compile the production binary with full optimizations and expose it globally across your system path environment:

Clone the repository
```powershell
git clone [https://github.com/botchx86/syskit.git](https://github.com/botchx86/syskit.git)
cd syskit
```
Compile and install directly into your system binary path
```
cargo install --path .
```
Method 2: Manual Artifact Compilation
To manually build the optimized standalone binary executable without global installation:

Bash
```
cargo build --release
```
The production artifact will be available at ./target/release/syskit.exe (Windows) or ./target/release/syskit (UNIX).

Command Reference
When initialized without arguments, syskit drops users into a colorized, high-contrast console matrix detailing machine states alongside runtime options.
```
Usage: syskit [COMMAND]

Commands:
  ps        List running processes
  kill      Terminate a process by name
  sysinfo   Show system diagnostics and uptime
  ping      Check a remote host connection
  scan      Scan target ports for connectivity
  ipinfo    Fetch public IP geolocation information
  download  Download a remote file
  hash      Compute cryptographic signatures
  encrypt   Encrypt a file using AES-256-GCM
  decrypt   Decrypt an encrypted file
  wipe      Securely shred a file from storage
```
Operational Examples
System Process Auditing
```PowerShell
syskit kill --target "malicious_process.exe"
```
Asynchronous Port Connectivity Sweeps
```PowerShell
syskit scan --ip "192.168.1.1" --ports "80-443"
```
Compute data hash signatures
```PowerShell
syskit hash --target "./sensitive_report.pdf" --file
```
Protect a target payload via authenticated encryption
```
syskit encrypt --file "./database.db"
```
Defensive Storage Shredding
```PowerShell
syskit wipe --file "./passwords.txt" --passes 5
```
Cryptographic Specification
Cipher Architecture: Advanced Encryption Standard (AES) operating in Galois/Counter Mode (GCM).

Key Length: 256-bit high-entropy secret generation keys.

Shredding Algorithm: Enforces a secure multipass pseudorandom structural write sequence, overwriting targeted disk addresses to permanently destroy underlying magnetic and solid-state indicators.

License
This project is deployed under the terms of the MIT License. For further information, review the explicit terms documented inside the LICENSE asset.
