# DNP3 Timming Attacks

## Overview

`DNP3Attacks` is a Rust-based command-line tool designed to synchronize time with a DNP3 outstation. It allows users to specify a desired timestamp for synchronization, either through a command-line argument or an interactive prompt. The tool leverages the `dnp3` crate to communicate with the outstation over TCP.

## Features

- Synchronize time with a DNP3 outstation using a custom timestamp.
- Supports both command-line and interactive input for specifying the timestamp.
- Configurable TCP connection to the outstation.
- Implements a custom `AssociationHandler` to provide the desired timestamp.

## Requirements

- Rust (edition 2021)
- Cargo

## Dependencies

The project uses the following Rust crates:
- `tokio` (for asynchronous runtime)
- `dnp3` (for DNP3 protocol communication)
- `anyhow` (for error handling)
- `windows` (for Windows-specific functionality)
- `chrono` (for date and time handling)
- `clap` (for command-line argument parsing)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/DNP3Attacks.git
   cd DNP3Attacks
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the executable:
   ```bash
   ./target/release/dnp3-time-sync
   ```

## Usage

### Command-Line Arguments

```bash
dnp3-time-sync [OPTIONS]
```

#### Options:
- `-i`, `--ip`: Outstation IP address and port (default: `10.152.152.152:20000`).
- `-t`, `--time`: Desired date & time in `YYYY-MM-DD HH:MM:SS` format (UTC).

### Examples

1. Synchronize time using a specific timestamp:
   ```bash
   dnp3-time-sync --ip 192.168.1.100:20000 --time "2023-10-01 12:00:00"
   ```

2. Synchronize time interactively:
   ```bash
   dnp3-time-sync --ip 192.168.1.100:20000
   ```

## How It Works

1. The tool parses the desired timestamp from the command-line argument or prompts the user interactively.
2. It establishes a TCP connection to the specified DNP3 outstation.
3. A custom `AssociationHandler` provides the desired timestamp to the outstation.
4. The tool sends a LAN time synchronization command to the outstation.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## Acknowledgments

This project uses the [dnp3](https://crates.io/crates/dnp3) crate for DNP3 protocol communication. Special thanks to the Rust community for their support and resources.
