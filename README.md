# Rust Workshop

## Examples on Rust Playground

* Concurrency Examples
  * [thread_new]() - Spawn a new thread and block main thread until it's completed
  * [thread_move]() - Demonstrate how ownership works with threads
  * [thread_message_passing]() - Establish a multi-producer, single-consumer channel to pass values between threads
  * [mutex]() - Demonstrate synchronization type `Mutex`
  * [arc]() - Demonstrate synchronization type `Arc` (used for reference counting)
  * [thread_mutex_arc]() - Demonstrate shared-state concurrency using synchronization types

## Setup Rust on Local Machine

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

## Running the examples on local machine

You can run any of the code examples by using `cargo run --example [example]`.

To run the mutex example, execute the following commands:

```bash
cargo run --example mutex
```
