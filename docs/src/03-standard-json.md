# Standard JSON

Standard JSON is a protocol for interaction with the **solx** and **solc** compilers. This protocol must be implemented by toolkits such as Hardhat and Foundry.

The protocol uses two data formats for communication: [input JSON](#input-json) and [output JSON](#output-json).



## Usage

Input JSON can be provided by-value via the `--standard-json` option:

```shell
solx --standard-json './input.json'
```

Alternatively, the input JSON can be fed to **solx** via *stdin*:

```shell
cat './input.json' | solx --standard-json
```

> For the sake of interface unification, **solx** will always return with exit code 0 and have its standard JSON output printed to *stdout*.
> It differs from **solc** that may return with exit code 1 and a free-formed error in some cases, such as when the standard JSON input file is missing, even though [the **solc** documentation claims otherwise](https://docs.soliditylang.org/en/latest/using-the-compiler.html#compiler-input-and-output-json-description).



## Input JSON

The input JSON provides the compiler with the source code and settings for the compilation. The example below serves as the specification of the input JSON format.

This format introduces several **solx**-specific parameters such as `settings.optimizer.sizeFallback`. These parameters are marked as `solx-only`.

On the other hand, parameters that are not mentioned here but are parts of **solc** standard JSON protocol have no effect in **solx**.

```javascript
{
  // Required: Source code language.
  // Currently supported: "Solidity", "Yul", "LLVM IR".
  "language": "Solidity",
  // Required: Source code files to compile.
  // The keys here are the "global" names of the source files. Imports can be using other file paths via remappings.
  "sources": {
    // In source file entry, either but not both "urls" and "content" must be specified.
    "myFile.sol": {
      // Required (unless "content" is used): URL(s) to the source file.
      "urls": [
        // In Solidity mode, directories must be added to the command-line via "--allow-paths <path>" for imports to work.
        // It is possible to specify multiple URLs for a single source file. In this case the first successfully resolved URL will be used.
        "/tmp/path/to/file.sol"
      ],
      // Required (unless "urls" is used): Literal contents of the source file.
      "content": "contract settable is owned { uint256 private x = 0; function set(uint256 _x) public { if (msg.sender == owner) x = _x; } }"
    }
  },

  // Required: Compilation settings.
  "settings": {
    // Optional: Optimizer settings.
    "optimizer": {
      // Optional, solx-only: Set the LLVM optimizer level.
      // Available options:
      // -0: do not optimize, currently unsupported
      // -1: basic optimizations for gas usage
      // -2: advanced optimizations for gas usage
      // -3: all optimizations for gas usage
      // -s: basic optimizations for bytecode size
      // -z: all optimizations for bytecode size
      // Default: 3.
      "mode": "3",
      // Optional, solx-only: Re-run the compilation with "mode": "z" if the initial compilation exceeds the EVM bytecode size limit.
      // Used on a per-contract basis and applied automatically, so some contracts will end up compiled in the initial mode, and others with "mode": "z".
      // Only activated if "mode" is set to "3", which is the default optimization mode.
      // Default: false.
      "sizeFallback": false
    },

    // Optional: Sorted list of remappings.
    // Important: Only used with Solidity input.
    "remappings": [ ":g=/dir" ],
    // Optional: Addresses of the libraries.
    // If not all library addresses are provided here, it will result in unlinked bytecode files that will require post-compile-time linking before deployment.
    // Important: Only used with Solidity, Yul, and LLVM IR input.
    "libraries": {
      // The top level key is the name of the source file where the library is used.
      // If remappings are used, this source file should match the global path after remappings were applied.
      "myFile.sol": {
        // Source code library name and address where it is deployed.
        "MyLib": "0x123123..."
      }
    },

    // Optional: Version of EVM solc will produce IR for.
    // Affects type checking and code generation.
    // Can be "homestead", "tangerineWhistle", "spuriousDragon", "byzantium", "constantinople", "petersburg", "istanbul", "berlin", "london", "paris", "shanghai", "cancun" or "prague".
    // Only used with Solidity, and only affects Yul and EVM assembly codegen. For instance, with version "cancun", solc will produce `MCOPY` instructions, whereas with older EVM versions it will not.
    // Default: "cancun".
    "evmVersion": "cancun",
    // Optional: Select the desired output.
    // Default: no flags are selected, and no output is generated.
    "outputSelection": {
      "<path>": {
        // Available file-level options, must be listed under "<path>"."":
        "": [
          // AST of all source files.
          "ast"
        ],
        // Available contract-level options, must be listed under "<path>"."<name>":
        "<name>": [
          // Solidity ABI.
          "abi",
          // Metadata.
          "metadata",
          // Developer documentation (natspec).
          "devdoc",
          // User documentation (natspec).
          "userdoc",
          // Slots, offsets and types of the contract's state variables in storage.
          "storageLayout",
          // Slots, offsets and types of the contract's state variables in transient storage.
          "transientStorageLayout",
          // Yul produced by solc.
          "irOptimized",
          // Everything of the below.
          "evm",
          // Solidity function hashes.
          "evm.methodIdentifiers",
          // EVM assembly produced by solc.
          "evm.legacyAssembly",
          // Everything that starts with "evm.bytecode".
          "evm.bytecode",
          // Deploy bytecode produced by solx/LLVM.
          // As long as the solx bytecode linker is in experimental stage, all contracts will be compiled if this key is enabled for at least one contract.
          "evm.bytecode.object",
          // Deploy code assembly produced by solx/LLVM.
          "evm.bytecode.llvmAssembly",
          // Unsupported, but emitted as an empty object to preserve compatibility with some toolkits.
          "evm.bytecode.opcodes",
          // Unsupported, but emitted as an empty object to preserve compatibility with some toolkits.
          "evm.bytecode.sourceMap",
          // Unsupported, but emitted as an empty object to preserve compatibility with some toolkits.
          "evm.bytecode.functionDebugData",
          // Unsupported, but emitted as an empty object to preserve compatibility with some toolkits.
          "evm.bytecode.generatedSources",
          // Everything that starts with "evm.deployedBytecode".
          "evm.deployedBytecode",
          // Runtime bytecode produced by solx/LLVM.
          // As long as the solx bytecode linker is in experimental stage, all contracts will be compiled if this key is enabled for at least one contract.
          "evm.deployedBytecode.object",
          // Runtime code assembly produced by solx/LLVM.
          "evm.deployedBytecode.llvmAssembly",
          // Link references for linkers that are to resolve library addresses at deploy time.
          "evm.deployedBytecode.linkReferences",
          // Resolved automatically by solx/LLVM, but emitted as an empty object to preserve compatibility with some toolkits.
          "evm.deployedBytecode.immutableReferences",
          // Unsupported, but emitted as an empty object to preserve compatibility with some toolkits.
          "evm.deployedBytecode.opcodes",
          // Unsupported, but emitted as an empty object to preserve compatibility with some toolkits.
          "evm.deployedBytecode.sourceMap",
          // Unsupported, but emitted as an empty object to preserve compatibility with some toolkits.
          "evm.deployedBytecode.functionDebugData",
          // Unsupported, but emitted as an empty object to preserve compatibility with some toolkits.
          "evm.deployedBytecode.generatedSources"
        ]
      }
    },
    // Optional: Metadata settings.
    "metadata": {
      // Optional: Use the given hash method for the metadata hash that is appended to the bytecode.
      // Available options: "none", "ipfs".
      // Default: "ipfs".
      "bytecodeHash": "ipfs",
      // Optional: Use only literal content and not URLs.
      // Default: false.
      "useLiteralContent": true,
      // Optional: Whether to include CBOR-encoded metadata at the end of bytecode.
      // Default: true.
      "appendCBOR": true
    },
    // Optional: Enables the IR codegen in solc.
    "viaIR": true,

    // Optional, solx: Extra LLVM settings.
    "llvmOptions": [
      "-key", "value"
    ]
  }
}
```



## Output JSON

The output JSON contains all artifacts produced by **solx** and **solc** together. The example below serves as the specification of the output JSON format.

```javascript
{
  // Required: File-level outputs.
  "sources": {
    "sourceFile.sol": {
      // Required: Identifier of the source.
      "id": 1,
      // Optional: The AST object.
      // Corresponds to "ast" in the outputSelection settings.
      "ast": {/* ... */}
    }
  },

  // Required: Contract-level outputs.
  "contracts": {
    // The source name.
    "sourceFile.sol": {
      // The contract name.
      // If the language only supports one contract per file, this field equals to the source name.
      "ContractName": {
        // Optional: The Ethereum Contract ABI (object).
        // See https://docs.soliditylang.org/en/develop/abi-spec.html.
        // Corresponds to "abi" in the outputSelection settings.
        "abi": [/* ... */],
        // Optional: Storage layout (object).
        // Corresponds to "storageLayout" in the outputSelection settings.
        "storageLayout": {/* ... */},
        // Optional: Transient storage layout (object).
        // Corresponds to "transientStorageLayout" in the outputSelection settings.
        "transientStorageLayout": {/* ... */},
        // Optional: Contract metadata (string).
        // Corresponds to "metadata" in the outputSelection settings.
        "metadata": "/* ... */",
        // Optional: Developer documentation (natspec object).
        // Corresponds to "devdoc" in the outputSelection settings.
        "devdoc": {/* ... */},
        // Optional: User documentation (natspec object).
        // Corresponds to "userdoc" in the outputSelection settings.
        "userdoc": {/* ... */},
        // Optional: Yul produced by solc (string).
        // Corresponds to "irOptimized" in the outputSelection settings.
        "irOptimized": "/* ... */",
        // Optional: EVM target outputs.
        // Corresponds to "evm" in the outputSelection settings.
        "evm": {
          // Optional: EVM assembly produced by solc (object).
          // Corresponds to "evm.legacyAssembly" in the outputSelection settings.
          "legacyAssembly": {/* ... */},
          // Optional: List of function hashes (object).
          // Corresponds to "evm.methodIdentifiers" in the outputSelection settings.
          "methodIdentifiers": {
            // Mapping between the function signature and its hash.
            "delegate(address)": "5c19a95c"
          },
          // Optional: Deploy EVM bytecode.
          // Corresponds to "evm.bytecode" in the outputSelection settings.
          "bytecode": {
            // Optional: Bytecode (string).
            // Corresponds to "evm.bytecode.object" in the outputSelection settings.
            "object": "5b60806040525f341415601c5763...",
            // Optional: LLVM text assembly (string).
            // Corresponds to "evm.bytecode.llvmAssembly" in the outputSelection settings.
            "llvmAssembly": "/* ... */",
            // Optional: Link references for linkers that are to resolve library addresses at deploy time (object).
            // Corresponds to "evm.bytecode.linkReferences" in the outputSelection settings.
            "linkReferences": {/* ... */},
            // Optional: Always empty, included only to preserve compatibility with some toolkits (string).
            // Corresponds to "evm.bytecode.immutableReferences" in the outputSelection settings.
            "opcodes": {},
            // Optional: Always empty, Included only to preserve compatibility with some toolkits (string).
            // Corresponds to "evm.bytecode.sourceMap" in the outputSelection settings.
            "sourceMap": {},
            // Optional: Always empty, Included only to preserve compatibility with some toolkits (array).
            // Corresponds to "evm.bytecode.functionDebugData" in the outputSelection settings.
            "functionDebugData": {},
            // Optional: Always empty, Included only to preserve compatibility with some toolkits (object).
            // Corresponds to "evm.bytecode.generatedSources" in the outputSelection settings.
            "generatedSources": {}
          },
          // Optional: Runtime EVM bytecode.
          // Corresponds to "evm.deployedBytecode" in the outputSelection settings.
          "deployedBytecode": {
            // Optional: Bytecode (string).
            // Corresponds to "evm.deployedBytecode.object" in the outputSelection settings.
            "object": "5b60806040525f34141560145760...",
            // Optional: LLVM text assembly (string).
            // Corresponds to "evm.deployedBytecode.llvmAssembly" in the outputSelection settings.
            "llvmAssembly": "/* ... */",
            // Optional: Link references for linkers that are to resolve library addresses at deploy time (object).
            // Corresponds to "evm.deployedBytecode.linkReferences" in the outputSelection settings.
            "linkReferences": {/* ... */},
            // Optional: Resolved by LLVM automatically, so always returned as an empty object (object).
            // Included only to preserve compatibility with some toolkits.
            // Corresponds to "evm.deployedBytecode.immutableReferences" in the outputSelection settings.
            "immutableReferences": {},
            // Optional: Always empty, included only to preserve compatibility with some toolkits (string).
            // Corresponds to "evm.deployedBytecode.opcodes" in the outputSelection settings.
            "opcodes": {},
            // Optional: Always empty, Included only to preserve compatibility with some toolkits (string).
            // Corresponds to "evm.deployedBytecode.sourceMap" in the outputSelection settings.
            "sourceMap": {},
            // Optional: Always empty, Included only to preserve compatibility with some toolkits (array).
            // Corresponds to "evm.deployedBytecode.functionDebugData" in the outputSelection settings.
            "functionDebugData": {},
            // Optional: Always empty, Included only to preserve compatibility with some toolkits (object).
            // Corresponds to "evm.deployedBytecode.generatedSources" in the outputSelection settings.
            "generatedSources": {}
          }
        }
      }
    }
  },

  // Optional: Unset if no messages were emitted.
  "errors": [
    {
      // Optional: Location within the source file.
      // Unset if the error is unrelated to input sources.
      "sourceLocation": {
        /// Required: The source path.
        "file": "sourceFile.sol",
        /// Required: The source location start. Equals -1 if unknown.
        "start": 0,
        /// Required: The source location end. Equals -1 if unknown.
        "end": 100
      },
      // Required: Message type.
      // solc errors are listed at https://docs.soliditylang.org/en/latest/using-the-compiler.html#error-types.
      "type": "Error",
      // Required: Component the error originates from.
      "component": "general",
      // Required: Message severity.
      // Possible values: "error", "warning", "info".
      "severity": "error",
      // Optional: Unique code for the cause of the error.
      // Only solc produces error codes for now.
      // solx currently emits errors without codes, but they will be introduced soon.
      "errorCode": "3141",
      // Required: Message.
      "message": "Invalid keyword",
      // Required: Message formatted using the source location.
      "formattedMessage": "sourceFile.sol:100: Invalid keyword"
    }
  ]
}
```
