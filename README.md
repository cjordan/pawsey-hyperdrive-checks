# pawsey-hyperdrive-checks

Some little bits of code used to verify that
[`hyperdrive`](https://github.com/MWATelescope/mwa_hyperdrive) is working
correctly.

## Usage
### hyperdrive-vis-gen-diff
This executable expects file outputs out of `mwa_hyperdrive simulate-vis` to be
in both the present working directory, and a directory "baseline". The files
should have a name like `hyperdrive_bandXX.bin`,
e.g. `hyperdrive_band01.bin`. `hyperdrive-vis-gen-diff` will compare all the
files it can, and report the maximum difference between all pairs. If the
difference is too large (0.001), then the executable will exit with code -1.

## Installation
<details>

### Prerequisites
<details>

- A Rust compiler

  `https://www.rust-lang.org/tools/install`

</details>

### Compilation
- Compile the source code with

    `cargo build --release`

  The binaries are then available in

    `./target/release/`

  On the same system, the binaries can be copied and used anywhere you like!

  To make static binaries, compile with

    `cargo build --release --target x86_64-unknown-linux-musl`

  You may need to run `rustup target add x86_64-unknown-linux-musl` first. The
  executables in `./target/x86_64-unknown-linux-musl/` should now run forever
  without any dependencies. Nice.

    </details> </details>

## Troubleshooting

Report the version of the software used, your usage and the program output in a
new GitHub issue.
