<div align="center">
  <img src=".github/assets/logo.png" alt="solx logo" />
</div>

# Optimizing Solidity Compiler

**solx** is a new optimizing compiler for EVM developed by [Matter Labs](https://matter-labs.io/).

> [!WARNING]  
> The project is in alpha state and not suitable for production use.

**solx** passes [our test suite](https://github.com/matter-labs/era-compiler-tester), which includes:

- [tests](https://github.com/ethereum/solidity/tree/develop/test/libsolidity/semanticTests) from the **solc** project
- [real-life projects](https://github.com/matter-labs/era-compiler-tests/tree/main/solidity/complex/defi) such as UniswapV2 and Mooniswap
- [additional tests](https://github.com/matter-labs/era-compiler-tests/tree/main/solidity) written by the **solx** team

Our pool of tests is updated with every **solc** release. Arbitrary contracts are expected to compile correctly, but some may be temporary affected by stack-too-deep errors. Benchmarks indicate that **solx** generates larger code than **solc**, but reduces the gas consumption on average.

Our mid-term goals:

- support EOF
- further reduce gas usage and bytecode size
- eliminate the need for inline assembly for efficiency
- emit [ethdebug](https://ethdebug.github.io/format/index.html) for optimized code
- run more tests of real-life projects in CI

## Installation

For the detailed installation and usage guide, visit [our comprehensive documentation](https://matter-labs.github.io/solx/latest/#installation).

## Demo

Check out [this repository](https://github.com/popzxc/solx_demo) to see a demo of the current state of the compiler.

## Architecture

**solx** consists of three main parts:

1. **solx** executable from this repository. The repository also contains parts of the compiler front end: Yul and EVM assembly translators.
2. [era-solidity](https://github.com/matter-labs/era-solidity/), an LLVM-friendly fork of [the Solidity compiler](https://github.com/ethereum/solidity),
  that emits Yul and EVM assembly for **solx**. Despite the repository name, it is not directly related to either ZKsync or ZKsync Era.
3. [era-compiler-llvm](https://github.com/matter-labs/era-compiler-llvm), a fork of [the LLVM project](https://github.com/llvm/llvm-project)
  with an EVM target developed by the **solx** team.

The most important part of the project is the EVM target in LLVM. You can find its sources [here](https://github.com/matter-labs/era-compiler-llvm/tree/main/llvm/lib/Target/EVM).

## Documentation

The **solx** documentation is powered by [GitHub Pages](https://matter-labs.github.io/solx/latest/).

**solx** documentation is provided as an [mdBook](https://github.com/rust-lang/mdBook), and its sources available in the `docs/` directory.
To build the book, follow the [instructions](./docs/README.md).

Alternatively, you may check out its Markdown representation in [this repository](./docs/src/).

See also:

- [Solidity Documentation](https://docs.soliditylang.org/en/latest/)
- [LLVM Documentation](https://llvm.org/docs/)

## Testing

To run the unit and CLI tests, execute `cargo test` at the repository root.

## Troubleshooting

If you have multiple LLVM builds in your system, ensure that you choose the correct one to build the compiler.
The environment variable `LLVM_SYS_191_PREFIX` sets the path to the directory with LLVM build artifacts, which typically ends with `target-llvm/build-final`.
For example:

```shell
export LLVM_SYS_191_PREFIX="${HOME}/src/solx/target-llvm/build-final"
```

If you suspect that the compiler is not using the correct LLVM build, check by running `set | grep LLVM`, and reset all LLVM-related environment variables.

For reference, see [llvm-sys](https://crates.io/crates/llvm-sys) and [Local LLVM Configuration Guide](https://llvm.org/docs/GettingStarted.html#local-llvm-configuration).

## License

- Crates **solx** and **solx-solc** are licensed under [GNU General Public License v3.0](./solx/LICENSE.txt)
- Crates **solx-standard-json**, **solx-evm-assembly**, **solx-yul** are licensed under the terms of either
  - Apache License, Version 2.0 ([LICENSE-APACHE](./solx-standard-json/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
  - MIT license ([LICENSE-MIT](./solx-standard-json/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
- [`era-solidity`](https://github.com/matter-labs/era-solidity/) is licensed under [GNU General Public License v3.0](https://github.com/matter-labs/era-solidity/blob/0.8.30/LICENSE.txt)
- [`era-compiler-llvm`](https://github.com/matter-labs/era-compiler-llvm) is licensed under the terms of Apache License, Version 2.0 with LLVM Exceptions, ([LICENSE](https://github.com/matter-labs/era-compiler-llvm/blob/main/LICENSE) or https://llvm.org/LICENSE.txt)

## Contact Us

Email us at [solx@matterlabs.dev](mailto:solx@matterlabs.dev).
