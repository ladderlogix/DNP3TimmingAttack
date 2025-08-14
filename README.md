# DNP3 Time Sync Tool (`dnp3time`)

`dnp3time` is a CLI tool designed to synchronize time on systems using the Distributed Network Protocol (DNP3). It enables users to send time synchronization commands to DNP3 outstations with either user-specified or interactively provided timestamps.

## Features
- Synchronize time to DNP3 outstations with user-defined timestamps.
- Interactive prompting for timestamp entry.
- Support for LAN time-sync procedure per DNP3 standards.
- Cross-platform builds for Linux and Windows.

## Installation
`dnp3time` is available on [crates.io](https://crates.io/crates/dnp3time). You can easily install it using `cargo`:

```bash
cargo install dnp3time
```

This will install the `dnp3time` binary and make it available globally on your system.

### From Source
Alternatively, you can build the tool from source. First, clone the repository and use cargo to build it:

```bash
git clone https://github.com/ladderlogix/DNP3TimmingAttack.git
cd DNP3TimmingAttack
cargo build --release
```

The binary will be located in the `target/release/` directory.

### Prebuilt Binaries
Prebuilt binaries for Windows and Linux are available on the [Releases](https://github.com/ladderlogix/DNP3TimmingAttack/releases) page. You can download them directly instead of building the tool manually.

## Usage
Run the tool with the following command:
```bash
dnp3time --ip <outstation_ip:port> --time <YYYY-MM-DD HH:MM:SS>
```

Example:
```bash
dnp3time --ip 10.152.152.152:20000 --time "2023-11-12 14:23:00"
```

Alternatively, you can run the command interactively without the `--time` flag to input a timestamp at runtime:
```bash
dnp3time --ip 10.152.152.152:20000
```

### Arguments
- `--ip` (`-i`): The IP address and port of the DNP3 outstation (default: `10.152.152.152:20000`).
- `--time` (`-t`): The target date and time in the format `YYYY-MM-DD HH:MM:SS` (optional).

### Example Workflow
A typical use case involves specifying the outstation's IP address and port along with the desired timestamp. If the timestamp is omitted, the tool will prompt for it interactively. This makes the tool flexible for both pre-configured and ad-hoc time synchronization.

## CI/CD Workflow
This repository includes CI/CD pipelines via GitHub Actions for:
- Building binaries for Windows and Linux.
- Code signing for Windows binaries using **Azure Trusted Signing**.
- Creating releases and publishing prebuilt binaries to GitHub Releases.
- Publishing the crate to [crates.io](https://crates.io).

See the GitHub Actions workflow file at `.github/workflows/main.yml` for more details.

## License
This project is licensed under the [AGPL-3.0](LICENSE).

## Contributing
Interested in contributing? See the [CONTRIBUTING.md](CONTRIBUTING.md) file for details.