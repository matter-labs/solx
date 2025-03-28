<div align="center">
  <img src=".github/assets/logo.png" alt="solx logo" />
</div>

# `solx`, an optimizing Solidity compiler

It passes [our test suite](https://github.com/matter-labs/era-compiler-tests), which includes tests from the `solc` project and real-world projects such as UniswapV2 and Mooniswap. All `solc` tests are updated with each new `solc` release. Arbitrary contracts are expected to compile correctly, except for stack-too-deep issues. Benchmarks indicate that `solx` generates larger code than `solc` but reduces gas consumption on average.

Our mid-term goals:

- Support EOF.
- Further reduce gas consumption of EVM code as well as code size.
- Eliminate the need for inline assembly for efficiency.
- Emit `ethdebug` for optimized code.
- Run more real-life smart-contracts in CI.

## Installation

You can get existing pre-releases from the [Releases](https://github.com/matter-labs/solx/releases) GitHub page.
Or, you can take a build used in [solx_demo](https://github.com/popzxc/solx_demo):

- [Linux/AMD64](https://github.com/matter-labs/solx/releases/download/d5a98e5/solx-linux-amd64-gnu)
- [Linux/Arm64](https://github.com/matter-labs/solx/releases/download/d5a98e5/solx-linux-arm64-gnu)
- [MacOS](https://github.com/matter-labs/solx/releases/download/d5a98e5/solx-macosx)

## Usage

We recommend using `solx` via [foundry](https://github.com/foundry-rs/foundry). It behaves the same way as
`solc-0.8.29`, so you can download the binary and specify:

```toml
[profile.solx]
solc_version = "/path/to/solx"
```

It might work with `hardhat` as well, but this has not been tested yet.

Otherwise, the interface is _mostly_ compatible with `solc`, so you can use it via CLI or standard JSON.

## Demo

Check [this repository](https://github.com/popzxc/solx_demo) to see a demo of the current state of the compiler.

## Architecture

`solx` consists of three main parts:

- `solx` executable, present in this repository. This repository also contains a part of the compiler front-end (Yul and EVMLA lowering).
- [`era-solidity`](https://github.com/matter-labs/era-solidity/), a fork of the [solidity compiler](https://github.com/ethereum/solidity),
  which contains the compiler front-end. Despite the repository name, it is not directly related to either ZKsync or ZKsync Era.
- [`era-compiler-llvm`](https://github.com/matter-labs/era-compiler-llvm), a fork of [`llvm-project`](https://github.com/llvm/llvm-project)
  with added EVM target.

The most important part of the project is EVM target in LLVM, you can find its sources [here](https://github.com/matter-labs/era-compiler-llvm/tree/main/llvm/lib/Target/EVM).

## Testing

To run the unit and CLI tests, execute the following command from the repository root:

```shell
cargo test
```

## Troubleshooting

If you have multiple LLVM builds in your system, ensure that you choose the correct one to build the compiler.
The environment variable `LLVM_SYS_170_PREFIX` sets the path to the directory with LLVM build artifacts, which typically ends with `target-llvm/build-final`.
For example:

```shell
export LLVM_SYS_170_PREFIX="${HOME}/src/solx/target-llvm/build-final"
```

If you suspect that the compiler is not using the correct LLVM build, check by running `set | grep LLVM`, and reset all LLVM-related environment variables.

For reference, see [llvm-sys](https://crates.io/crates/llvm-sys) and [Local LLVM Configuration Guide](https://llvm.org/docs/GettingStarted.html#local-llvm-configuration).

## License

**solx** is licensed under [GNU General Public License v3.0](LICENSE.txt).

- [`era-compiler-solidity`](https://github.com/matter-labs/era-solidity/) is licensed under [GNU General Public License v3.0](https://github.com/matter-labs/era-solidity/blob/0.8.28/LICENSE.txt).
- [`era-compiler-llvm`](https://github.com/matter-labs/era-compiler-llvm) is licensed under the terms of Apache License, Version 2.0 with LLVM Exceptions, ([LICENSE](https://github.com/matter-labs/era-compiler-llvm/blob/main/LICENSE) or https://llvm.org/LICENSE.txt)

## Resources

- [Solidity documentation](https://docs.soliditylang.org/en/latest/)

## Official Links

- [GitHub](https://github.com/matter-labs)
