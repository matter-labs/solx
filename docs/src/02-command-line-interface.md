# Command Line Interface (CLI)

The CLI of **solx** is designed with resemblance to the CLI of **solc**. There are several main input/output (I/O) modes in the **solx** interface:

- [Basic CLI](#basic-cli)
- [Standard JSON](./03-standard-json.md)

The basic CLI and combined JSON modes are more light-weight and suitable for calling from the shell. The standard JSON mode is similar to client-server interaction, thus more suitable for using from other applications.

> All toolkits using **solx** must be operating in standard JSON mode and follow [its specification](./03-standard-json.md).
> It will make the toolkits more robust and future-proof, as the standard JSON mode is the most versatile and used for the majority of popular projects.

This page focuses on the basic CLI mode. For more information on the standard JSON mode, see [the corresponding page](./03-standard-json.md).



## Basic CLI

Basic CLI mode is the simplest way to compile a file with the source code.

To compile a basic Solidity contract, run the simple example from [the *--bin* section](#--bin).

The rest of this section describes the available CLI options and their usage. You may also check out `solx --help` for a quick reference.



### `--bin`

Enables the output of compiled bytecode. The following command compiles a Solidity file and prints the bytecode:

```bash
solx './Simple.sol' --bin
```

Output:

```text
======= Simple.sol:Simple =======
Binary:
0000008003000039000000400030043f0000000100200190000000130000c13d...
```

It is possible to dry-run the compilation without writing any output. To do this, simply omit `--bin` and other output options:

```bash
solx './Simple.sol'
```

Output:

```text
Compiler run successful. No output requested. Use flags --metadata, --asm, --bin.
```



### Input Files

**solx** supports multiple input files. The following command compiles two Solidity files and prints the bytecode:

```bash
solx './Simple.sol' './Complex.sol' --bin
```

[Solidity import remappings](https://docs.soliditylang.org/en/latest/path-resolution.html#import-remapping) are passed in the way as input files, but they are distinguished by a `=` symbol between source and destination. The following command compiles a Solidity file with a remapping and prints the bytecode:

```bash
solx './Simple.sol' 'github.com/ethereum/dapp-bin/=/usr/local/lib/dapp-bin/' --bin
```

**solx** does not handle remappings itself, but only passes them through to *solc*.
Visit [the **solc** documentation](https://docs.soliditylang.org/en/latest/using-the-compiler.html#base-path-and-import-remapping) to learn more about the processing of remappings.



### `--libraries`

Specifies the libraries to link with compiled contracts. The option accepts multiple string arguments. The safest way is to wrap each argument in single quotes, and separate them with a space.

The specifier has the following format: `<ContractPath>:<ContractName>=<LibraryAddress>`.

Usage:

```bash
solx './Simple.sol' --bin --libraries 'Simple.sol:Test=0x1234567890abcdef1234567890abcdef12345678'
```

There are two ways of linking libraries:
1. At compile time, immediately after the contract is compiled.
2. At deploy time (a.k.a. post-compile time), right before the contract is deployed.

The use case above describes linking at compile time. For linking at deploy time, see the [linker documentation](./05-linker.md).



### `--base-path`, `--include-path`, `--allow-paths`

These options are used to specify Solidity import resolution settings. They are not used by **solx** and only passed through to **solc** like import remappings.

Visit [the **solc** documentation](https://docs.soliditylang.org/en/latest/path-resolution.html) to learn more about the processing of these options.



### `--metadata`

Enables the output of contract metadata. The metadata is a JSON object that contains information about the contract, such as its name, source code hash, the list of dependencies, compiler versions, and so on.

The **solx** metadata format is compatible with the [Solidity metadata format](https://docs.soliditylang.org/en/latest/metadata.html#contract-metadata). This means that the metadata output can be used with other tools that support Solidity metadata. Essentially, **solc** metadata is a part of **solx** metadata, and it is included as `source_metadata` without any modifications.

Usage:

```bash
solx './Simple.sol' --metadata
```

Output:

```text
======= Simple.sol:Simple =======
Metadata:
{"llvm_options":[],"optimizer_settings":{"is_debug_logging_enabled":false,"is_fallback_to_size_enabled":false,"is_verify_each_enabled":false,"level_back_end":"Aggressive","level_middle_end":"Aggressive","level_middle_end_size":"Zero"},"solc_version":"x.y.z","solc_zkvm_edition":null,"source_metadata":{...},"zk_version":"x.y.z"}
```



### `--output-dir`

Specifies the output directory for build artifacts. Can only be used in [basic CLI](#basic-cli) and [combined JSON](./04-combined-json.md) modes.

Usage in basic CLI mode:

```bash
solx './Simple.sol' --bin --asm --metadata --output-dir './build/'
ls './build/Simple.sol'
```

Output:

```text
Compiler run successful. Artifact(s) can be found in directory "build".
...
Test.zasm       Test.zbin       Test_meta.json
```

Usage in combined JSON mode:

```bash
solx './Simple.sol' --combined-json 'bin,asm,metadata' --output-dir './build/'
ls './build/'
```

Output:

```text
Compiler run successful. Artifact(s) can be found in directory "build".
...
combined.json
```



### `--overwrite`

Overwrites the output files if they already exist in the output directory. By default, **solx** does not overwrite existing files.

Can only be used in combination with the [`--output-dir`](#--output-dir) option.

Usage:

```bash
solx './Simple.sol' --combined-json 'bin,asm,metadata' --output-dir './build/' --overwrite
```

If the `--overwrite` option is not specified and the output files already exist, **solx** will print an error message and exit:

```text
Error: Refusing to overwrite an existing file "build/combined.json" (use --overwrite to force).
```



### `--version`

Prints the version of **solx** and the hash of the LLVM commit it was built with.

Usage:

```bash
solx --version
```



### `--help`

Prints the help message.

Usage:

```bash
solx --help
```



## Other I/O Modes

> The mode-altering CLI options are mutually exclusive. This means that only one of the options below can be enabled at a time:
> - `--standard-json`
> - `--yul`
> - `--llvm-ir`



### `--standard-json`

For the standard JSON mode usage, see the [Standard JSON](./03-standard-json.md) page.



## **solx** Compilation Settings

The options in this section are only configuring the **solx** compiler and do not affect the underlying **solc** compiler.



### `--optimization / -O`

Sets the optimization level of the LLVM optimizer. Available values are:

| Level | Meaning                      | Hints                                            |
|:------|:-----------------------------|:-------------------------------------------------|
| 0     | No optimization              | Currently not supported
| 1     | Performance: basic           | For optimization research
| 2     | Performance: default         | For optimization research
| 3     | Performance: aggressive      | Best performance for production
| s     | Size: default                | For optimization research
| z     | Size: aggressive             | Best size for contracts with size constraints

For most cases, it is fine to keep the default value of `3`. You should only use the level `z` if you are ready to deliberately sacrifice performance and optimize for size.

> Large contracts may hit the EVM bytecode size limit. In this case, it is recommended using the [`--optimization-size-fallback`](#--optimization-size-fallback) option rather than setting the level to `z`.



### `--optimization-size-fallback`

Sets the optimization level to `z` for contracts that failed to compile due to overrunning the bytecode size constraints.

Under the hood, this option automatically triggers recompilation of contracts with level `z`. Contracts that were successfully compiled with [the original `--optimization` setting](#--optimization---o) are not recompiled.

> It is recommended to have this option always enabled to prevent compilation failures due to bytecode size constraints. There are no known downsides to using this option.



### `--metadata-hash`

Specifies the hash function used for contract metadata.

Usage with `ipfs`:

```bash
solx './Test.sol' --bin --metadata-hash 'ipfs'
```

Output with `ipfs`:

```text
======= .../Test.sol:Test =======
Binary:
00000001002001900000000c0000613d0000008001000039000000400010043f0000000001000416000000000001004b0000000c0000c13d00000020010000390000010000100443000001200000044300000005010000410000000f0001042e000000000100001900000010000104300000000e000004320000000f0001042e0000001000010430000000000000000000000000000000000000000000000000000000020000000000000000000000000000004000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a1646970667358221220aa6c03adc327b2cf98010f155f9849134e325f7a1e2cffd5b99d832a7dba5082002a
```

Note that a lot of padding is added before the `ipfs` hash to make the bytecode size an odd number of 32-byte words.



### `--llvm-options`

Specifies additional options for the LLVM framework. The argument must be a single quoted string following a `=` separator.

Usage:

```bash
solx './Simple.sol' --bin --llvm-options='-key=value'
```

> The `--llvm-options` option is experimental and must only be used by experienced users. All supported options will be documented in the future.



## **solc** Compilation Settings

The options in this section are only configuring **solc**, so they are passed directly to its child process, and do not affect the **solx** compiler.



### `--codegen`

Specifies the **solc** codegen. The following values are allowed:

| Value | Description                  | Defaults                           |
|:------|:-----------------------------|:-----------------------------------|
| evmla | EVM legacy assembly          | **solc** default for EVM/L1          |
| yul   | Yul a.k.a. IR                | **solx** default for ZKsync        |

> **solc** uses the `evmla` codegen by default. However, **solx** uses the `yul` codegen by default for historical reasons.
> Codegens are not equivalent and may lead to different behavior in production.
> Make sure that this option is set to `evmla` if you want your contracts to behave as they would on L1.
> For codegen differences, visit the [solc IR breaking changes page](https://docs.soliditylang.org/en/latest/ir-breaking-changes.html).
> **solx** is going to switch to the `evmla` codegen by default in the future in order to have more parity with L1.

Usage:

```bash
solx './Simple.sol' --bin --codegen 'evmla'
```



### `--evm-version`

Specifies the EVM version **solc** will produce artifacts for. Only artifacts such as Yul and EVM assembly are known to be affected by this option. For instance, if the EVM version is set to *cancun*, then Yul and EVM assembly may contain `MCOPY` instructions, so no calls to the Identity precompile (address `0x04`) will be made.

> EVM version only affects IR artifacts produced by **solc** and does not affect EVM bytecode produced by **solx**.

The default value is chosen by **solc**. For instance, **solc** v0.8.24 and older use *shanghai* by default, whereas newer ones use *cancun*.

The following values are allowed, however have in mind that newer EVM versions are only supported by newer versions of *solc*:
- homestead
- tangerineWhistle
- spuriousDragon
- byzantium
- constantinople
- petersburg
- istanbul
- berlin
- london
- paris
- shanghai
- cancun
- prague

Usage:

```bash
solx './Simple.sol' --bin --evm-version 'cancun'
```

For more information on how **solc** handles EVM versions, see its [EVM version documentation](https://docs.soliditylang.org/en/latest/using-the-compiler.html#setting-the-evm-version-to-target).



### `--metadata-literal`

Tells **solc** to store referenced sources as literal data in the metadata output.

> This option only affects the contract metadata output produced by **solc**, and does not affect artifacts produced by **solx**.

Usage:

```bash
solx './Simple.sol' --bin --metadata-literal
```



## Multi-Language Support

**solx** supports input in multiple programming languages:

- [Solidity](https://soliditylang.org/)
- [Yul](https://docs.soliditylang.org/en/latest/yul.html)
- [LLVM IR](https://llvm.org/docs/LangRef.html)

The following sections outline how to use **solx** with these languages.



### `--yul`

Enables the Yul mode. In this mode, input is expected to be in the Yul language. The output works the same way as with Solidity input.

Usage:

```bash
solx --yul './Simple.yul' --bin
```

Output:

```text
======= Simple.yul =======
Binary:
0000000100200190000000060000c13d0000002a01000039000000000010043f...
```



### `--llvm-ir`

Enables the LLVM IR mode. In this mode, input is expected to be in the LLVM IR language. The output works the same way as with Solidity input.

Unlike **solc**, **solx** is an LLVM-based compiler toolchain, so it uses LLVM IR as an intermediate representation. It is not recommended to write LLVM IR manually, but it can be useful for debugging and optimization purposes. LLVM IR is more low-level than Yul in the ZKsync compiler toolchain IR hierarchy, so **solc** is not used for compilation.

Usage:

```bash
solx --llvm-ir './Simple.ll' --bin
```

Output:

```text
======= Simple.ll =======
Binary:
000000000002004b000000070000613d0000002001000039000000000010043f...
```



## Debugging



### `--debug-output-dir`

Specifies the directory to store intermediate build artifacts. The artifacts can be useful for debugging and research.

The directory is created if it does not exist. If artifacts are already present in the directory, they are overwritten.

The intermediate build artifacts can be:

| Name            | Codegen         | File extension   |
|:----------------|:----------------|:-----------------|
| EVM Assembly    | evmla           | *evmla*          |
| EthIR           | evmla           | *ethir*          |  
| Yul             | yul             | *yul*            |
| LLVM IR         | evmla, yul      | *ll*             |

Usage:

```bash
solx './Simple.sol' --bin --debug-output-dir './debug/'
ls './debug/'
```

Output:

```text
Compiler run successful. No output generated.
...
Simple.sol.C.runtime.optimized.ll
Simple.sol.C.runtime.unoptimized.ll
Simple.sol.C.yul
Simple.sol.C.zasm
Simple.sol.Test.runtime.optimized.ll
Simple.sol.Test.runtime.unoptimized.ll
Simple.sol.Test.yul
Simple.sol.Test.zasm
```

The output file name is constructed as follows: `<ContractPath>.<ContractName>.<Modifiers>.<Extension>`.



### `--llvm-verify-each`

Enables the verification of the LLVM IR after each optimization pass. This option is useful for debugging and research purposes.

Usage:

```bash
solx './Simple.sol' --bin --llvm-verify-each
```



### `--llvm-debug-logging`

Enables the debug logging of the LLVM IR optimization passes. This option is useful for debugging and research purposes.

Usage:

```bash
solx './Simple.sol' --bin --llvm-debug-logging
```
