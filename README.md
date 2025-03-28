<div align="center">
  <img src=".github/assets/logo.png" alt="solx logo" />
</div>

# `solx`, an optimizing Solidity compiler

**solx** passes [our test suite](https://github.com/matter-labs/era-compiler-tester), which includes:

- [tests](https://github.com/ethereum/solidity/tree/develop/test/libsolidity/semanticTests) from the **solc** project
- [real-world projects](https://github.com/matter-labs/era-compiler-tests/tree/main/solidity/complex/defi) such as UniswapV2 and Mooniswap
- [additional tests](https://github.com/matter-labs/era-compiler-tests/tree/main/solidity) written by the **solx** team

Our pool of tests is updated with every **solc** release. Arbitrary contracts are expected to compile correctly, but some may be temporary affected by stack-too-deep errors. Benchmarks indicate that **solx** generates larger code than **solc**, but reduces the gas consumption on average.

Our mid-term goals:

- support EOF
- further reduce gas consumption and bytecode size
- eliminate the need for inline assembly for efficiency
- emit [ethdebug](https://ethdebug.github.io/format/index.html) for optimized code
- run more real-life smart-contracts in CI

## Installation

You can download the existing pre-releases from the [Releases](https://github.com/matter-labs/solx/releases) GitHub page.
Or, you can take a build used in our [solx_demo](https://github.com/popzxc/solx_demo):

- [MacOS](https://github.com/matter-labs/solx/releases/download/d5a98e5/solx-macosx)
- [Linux/AMD64](https://github.com/matter-labs/solx/releases/download/d5a98e5/solx-linux-amd64-gnu)
- [Linux/Arm64](https://github.com/matter-labs/solx/releases/download/d5a98e5/solx-linux-arm64-gnu)

## Usage

We recommend using `solx` via [foundry](https://github.com/foundry-rs/foundry). It behaves the same way as
`solc-0.8.29`, so you can download the executable and specify:

```toml
[profile.solx]
solc_version = "/path/to/solx"
```

It might work with `hardhat` as well, but it has not been tested yet.

Otherwise, the interface is _mostly_ compatible with `solc`, so you can use it via CLI or standard JSON.

## Demo

Check out [this repository](https://github.com/popzxc/solx_demo) to see a demo of the current state of the compiler.

## Architecture

**solx** consists of three main parts:

1. **solx** executable from this repository. The repository also contains a part of the compiler front-end (Yul and EVM assembly lowering).
2. [era-solidity](https://github.com/matter-labs/era-solidity/), an LLVM-friendly fork of [the Solidity compiler](https://github.com/ethereum/solidity),
  that emits Yul and EVM assembly for **solx**. Despite the repository name, it is not directly related to either ZKsync or ZKsync Era.
3. [era-compiler-llvm](https://github.com/matter-labs/era-compiler-llvm), a fork of [the LLVM project](https://github.com/llvm/llvm-project)
  with an EVM target developed by the **solx** team.

The most important part of the project is the EVM target in LLVM. You can find its sources [here](https://github.com/matter-labs/era-compiler-llvm/tree/main/llvm/lib/Target/EVM).

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

## Contact Us

Email us at [solx@matterlabs.dev](mailto:solx@matterlabs.dev).
