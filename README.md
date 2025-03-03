# LLVM-based Solidity Compiler Toolchain

This repository contains an LLVM-based Compiler Toolchain for Solidity and Yul.

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

## Resources

- [Solidity documentation](https://docs.soliditylang.org/en/latest/)

## Official Links

- [GitHub](https://github.com/matter-labs)
