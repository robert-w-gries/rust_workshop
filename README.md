# Rust Workshop

Materials for presenting at Develop@Cisco

## Setup

### Mac

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Linux

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### Ubuntu/Debian

```
apt install build-essential
```

#### Centos/Fedora

```
yum install gcc
```

## Running the examples

You can run any of the code examples by using `cargo run --example [example]`.

To run the mutex example, execute the following commands:

```bash
cd example_project
cargo run --example mutex
```
