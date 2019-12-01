# Rust Workshop

## Examples on Rust Playground

* Examples for Type Safety 
  * [Mutability](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=68d88421f6c54365687ef12e4a1e7c04) - Variables are immutable by default
  
  * [Array Overflow_Rust](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=9a299b5171af1befe2bc0cdf66e6d315)
  
  * [Integer Overflow C](https://godbolt.org/z/AAz87P)
  * [Integer Overflow Rust](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=bb0e32e4647558124e2b8f8e5edf2113)

  * [Integer Underflow C](https://godbolt.org/z/_vBjjz)
  * [Integer Underflow](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=2a563fa32275ec177c02e5f4848d5de4)

  * [Character Overflow](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=781fb9a66fd76c327444b846a172de36)
  * [Character Overflow C](https://godbolt.org/z/QJwANA)


* Examples for Memory Safety

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
