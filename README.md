# personal-dpi
Rust app to analyse packets arriving on your machine.

## requirements
sudo apt-get install libpcap-dev (linux/debian)

brew install libpcap (macOS)

## run
elevated privileged needed for packet capture

cargo build

sudo sudo ./target/debug/personal-dpi 