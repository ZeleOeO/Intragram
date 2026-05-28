# Intragram

A Rust-based TCP Chat Implementation. Might add a couple new things later, this isn't finished

## Technologies

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust&logoColor=white)
![Cargo](https://img.shields.io/badge/Cargo-Build--Tool-red?logo=rust&logoColor=white)
![HashMap](https://img.shields.io/badge/State-Persistent--Ledger-blue)

## Prerequisites

- Rust Toolchain (rustc & cargo)
- TCP Client
- **Optional** - An IDE such as VSCode (with rust-analyzer) or IntelliJ Rust. I personally use my terminal with nvim

## Installation (Will work on both powershell and bash/zsh terminals)

1. Clone the repository:
   ```bash
   git clone https://github.com/ZeleOeO/Intragram.git
   ```

2. Navigate to the project directory:
   ```bash
   cd Intragram
   ```

3. Ensure Rust is installed by running `cargo --version`

### Run Application

1. Run the TCP Server with the default transaction vectors:
   ```bash
   cargo run
   ```

## Usage

If you want to test out the system, you can have one or more other TCP Clients up and run with `cargo run`

You'll see the message `Listening on Server 127.0.0.1:6821`

To *connect* to the server you can use:
```bash
telnet localhost 6821
```

OR (If you're on a Macbook):
```bash
nc localhost 6821
```

Now you can type and send messages to and fro, pretty neat

### TODO

- [ ] Implement a TUI
- [ ] Message Persistence
- [ ] Encryption
- [ ] Username and Presence Status
- [ ] Typing indicator (maybe????)
- [ ] Ability to find any existing server

## Tests

I don't have a massive test suite yet, but if i do come back to it, you can run the unit tests with this:
```shell
cargo test
```

## Steps to Contribute

Contributions are more than welcome, I'm still working on the opcode set though, so... keep that in mind or something.

1. Open an issue first so I can like keep track, but if that's too much stress that's fine too
2. Fork the Repository
3. Clone your fork
4. Create a new branch:
   ```bash
   git checkout -b your-branch-name
   ```
5. Make your change
6. Commit your change, please use [Conventional Commits](https://gist.github.com/qoomon/5dfcdf8eec66a051ecd85625518cfd13) if you can.
7. Push your change
8. Make a pull request and reference your issue

Please stick to idiomatic Rust patterns, don't mess up my already spaghetti code.
