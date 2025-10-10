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
//!                 "topics": [],
//!                 "values": []
//!             },
//!             {
//!                 "topics": [],
//!                 "values": ["0xdeadbeef"]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": ["-128", "-1"]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [
//!                     "127",
//!                     "0xdeadbeef2",
//!                     "2"
//!                 ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [
//!                     "0x20",
//!                     "3",
//!                     "0x6162630000000000000000000000000000000000000000000000000000000000"
//!                 ]
//!             },
//!             {
//!                 "topics": [],
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
    event Empty() anonymous;
    event OneWord(uint256 x) anonymous;
    event TwoWords(Str) anonymous;
    event ThreeWords(Str, Enum e) anonymous;
    event Dynamic(string) anonymous;
    event Complex(Str s, bytes, uint256[1], Enum[] e) anonymous;

    function test(uint256 number) public {
        emit Empty();
        emit OneWord(CONST);
        emit TwoWords(Str(-128, 2**256-1));
        emit ThreeWords(Str(127, number), Enum.C);
        emit Dynamic("abc");
        Enum[] memory enums = new Enum[](3);
        enums[0] = Enum.A;
        enums[1] = Enum.C;
        enums[2] = Enum.B;
        emit Complex(Str(-1, IMMUTABLE), storage_var, [uint256(0)], enums);
    }
}
