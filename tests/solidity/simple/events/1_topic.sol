//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "test",
//!             "calldata": [
//!                 "0xdeadbeef2"
//!             ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!         ],
//!         "events": [
//!             {
//!                 "topics": [
//!                     "0x3db2a12a1fcfb6e2e0de8666c64b3244166edb7b25b200204a8d0cca9b65cf37"
//!                 ],
//!                 "values": []
//!             },
//!             {
//!                 "topics": [
//!                     "0xffffffffffffffffffffffffffffffffffffffffffffffffff"
//!                 ],
//!                 "values": ["0xdeadbeef"]
//!             },
//!             {
//!                 "topics": [
//!                     "0xca3f9448a573966265a425c911e248f5d76acd7ab4f9b31f3ec2440038c63561"
//!                 ],
//!                 "values": ["-128", "-1"]
//!             },
//!             {
//!                 "topics": [
//!                     "0x83c737ad570e9f3e71e0d2800958e44770d812e92db2c1758626613d1e6ba514"
//!                 ],
//!                 "values": [
//!                     "127",
//!                     "0xdeadbeef2",
//!                     "2"
//!                 ]
//!             },
//!             {
//!                 "topics": [
//!                     "0x6a10ac5d5f69030d7fc69d2e2d9fa31c998637f1be5ca8a27b95e9f87372b7e4"
//!                 ],
//!                 "values": [
//!                     "0x20",
//!                     "3",
//!                     "0x6162630000000000000000000000000000000000000000000000000000000000"
//!                 ]
//!             },
//!             {
//!                 "topics": [
//!                     "0xff00000000000000000000000000000000000000000000000000000000000000"
//!                 ],
//!                 "values": [
//!                     "-1",
//!                     "0xdeadbeef3",
//!                     "0xa0",
//!                     "0",
//!                     "0xe0",
//!                     "5",
//!                     "0xdeadbeef4000000000000000000000000000000000000000000000000000000",
//!                     "3",
//!                     "0",
//!                     "2",
//!                     "1"
//!                 ]
//!             }
//!         ]
//!     }
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.0;
pragma abicoder v2;

contract Test {
    uint256 constant CONST = 0xdeadbeef;
    uint256 immutable IMMUTABLE;
    bytes storage_var;

    constructor() {
        IMMUTABLE = 0xdeadbeef3;
        storage_var = hex"0deadbeef4";
    }

    struct Str {
        int8 a;
        uint256 b;
    }
    enum Enum {
        A,
        B,
        C
    }
    event Empty();
    event OneWord(uint200 indexed i, uint256 x) anonymous;
    event TwoWords(Str);
    event ThreeWords(string indexed, Str, Enum e) anonymous;
    event Dynamic(string);
    event Complex(bytes1 indexed b, Str s, bytes, uint256[1], Enum[] e) anonymous;

    function test(uint256 number) public {
        emit Empty();
        emit OneWord(2**200-1, CONST);
        emit TwoWords(Str(-128, 2**256-1));
        emit ThreeWords("some string", Str(127, number), Enum.C);
        emit Dynamic("abc");
        Enum[] memory enums = new Enum[](3);
        enums[0] = Enum.A;
        enums[1] = Enum.C;
        enums[2] = Enum.B;
        emit Complex(bytes1(0xff), Str(-1, IMMUTABLE), storage_var, [uint256(0)], enums);
    }
}
