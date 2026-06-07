# 🦀 rtio

`rtio` is a lightweight serial/UART terminal tool written in `Rust`.

Inspired by [tio tool](https://github.com/tio/tio) but stripped one, and designed for viewing device logs, debugging embedded systems, and communicating with hardware over serial connections.

## Base features

- Easily connect to serial TTY devices
- Sensible defaults (`115200 8n1`)
- Interactive UART terminal
- Configurable baud rate
- Raw terminal mode
- Graceful exit shortcut
- RX logging to file
- List available serial devices
    - By device
        - Including topology ID and description
    - By ID
- Timestamp support
    - Per line in normal output mode
- Log to file
    - Manual naming of log file
    - Append to log file
Strip control characters and escape sequences

## Build from source

### Prerequisites

Install the latest stable Rust toolchain:

```bash
rustup update
```

Verify the installation:

```bash
cargo --version
rustc --version
```

### Linux

Build the release binary:

```bash
cargo build --release
```

The resulting executable will be located at:

```text
target/release/rtio
```

You may copy it to a directory in your `PATH`, for example:

```bash
sudo cp target/release/rtio /usr/local/bin/
```

> If `rtio` cannot open a serial device due to permission errors, make sure your user is a member of the `dialout` group:
>
> ```bash
> sudo usermod -aG dialout $USER
> ```
>
> Log out and log back in for the change to take effect.

### macOS

Build the release binary:

```bash
cargo build --release
```

The resulting executable will be located at:

```text
target/release/rtio
```

Optionally install it system-wide:

```bash
sudo cp target/release/rtio /usr/local/bin/
```

On Apple Silicon (M1/M2/M3), the default build target is `aarch64-apple-darwin`.

### Windows

Build the release binary:

```powershell
cargo build --release
```

The resulting executable will be located at:

```text
target\release\rtio.exe
```

You can run it directly:

```powershell
.\target\release\rtio.exe --help
```

### Verify the build

Run:

```bash
rtio --help
```

or directly from the build directory:

```bash
./target/release/rtio --help
```


The result bin is in `./target/release/` dir.

## Usage

Typical use is without options

```
$ rtio -d /dev/ttyUSB0
```

Help message:

```sh
$ rtio --help

Small serial/UART tool written in Rust.
In session, use 'Ctrl + T, Q' to exit from rtio.

Usage: rtio [OPTIONS] --dev <DEV>

Options:
  -d, --dev <DEV>
          tty-device

  -b, --baud <BAUD>
          Baud rate of the device

          [default: 115200]

  -l, --log <LOG>
          Log to file

  -t, --timestamp
          Prefix each new line with a timestamp

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

Exit shortcut like in `tio`:

```
Ctrl + T, Q
```

## Examples

#### 1. Typical use is without options

```
$ rtio -d /dev/ttyUSB0
```

#### 2. Login into SBC Linux-based deice

```
$ rtio -d /dev/ttyCH341USB1

[rtio 12:21:12] File logger started in charge.txt
[rtio 12:21:12] Prefix each new line with a timestamp enabled
[rtio 12:21:12] Connected to device /dev/ttyCH341USB1, baud rate 115200. Ctrl+T, Q to quit

#
# ls
server  qspi
# cd server/
# ls
# server.py
```

#### 3. Log ESP32-based device charging

```
$ rtio -d /dev/ttyACM0 -t -l charge.txt

[rtio 12:25:42] File logger started in charge.txt
[rtio 12:25:42] Prefix each new line with a timestamp enabled
[rtio 12:25:42] Connected to device /dev/ttyACM0, baud rate 115200. Ctrl+T, Q to quit
[rtio 12:25:42] Current battery level (charging): 99%
[rtio 12:25:42] Current battery level (charging): 99%
[rtio 12:25:43] Current battery level (charging): 99%
[rtio 12:25:44] Current battery level (charging): 99%
[rtio 12:25:45] Current battery level (charging): 100%
[rtio 12:25:46] Current battery level (charging): 100%
```

#### 4. List available serial devices

```
rtio --list

Found 2 available port(s):

Port name: /dev/ttyCH341USB0
  VID: 1a86
  PID: 7523
  MAN: 1a86

Port name: /dev/ttyACM0
  VID: 303a
  PID: 1001
  SER: 84:FC:E6:6B:83:28
  MAN: Espressif
```

## Authors

Maintained by [nullsych](https://github.com/nullsych).
