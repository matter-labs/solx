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
//!                     "0xf2c6b74a4e42434bf406d1452c6d967321daaad79270822b1e44a42b0aa6aa1f",
//!                     "8923892323892398"
//!                 ],
//!                 "values": []
//!             },
//!             {
//!                 "topics": [
//!                     "0xffffffffffffffffffffffffffffffffffffffffffffffffff",
//!                     "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80"
//!                 ],
//!                 "values": ["0xdeadbeef"]
//!             },
//!             {
//!                 "topics": [
//!                     "0x416f2e35c7259c015d56b337484f39360b06f1d0b53f129353beead6cc8c5d71",
//!                     "0xcc69885fda6bcc1a4ace058b4a62bf5e179ea78fd58a1ccd71c22cc9b688792f"
//!                 ],
//!                 "values": ["-128", "-1"]
//!             },
//!             {
//!                 "topics": [
//!                     "0x83c737ad570e9f3e71e0d2800958e44770d812e92db2c1758626613d1e6ba514",
//!                     "1"
//!                 ],
//!                 "values": [
//!                     "127",
//!                     "0xdeadbeef2",
//!                     "2"
//!                 ]
//!             },
//!             {
//!                 "topics": [
//!                     "0xb003e25404531c83a6d0bdc419158b8c07c1a5f3e81ab387366c8b7e572c1ae5",
//!                     "0xabcd000000000000000000000000000000000000000000000000000000000000"
//!                 ],
//!                 "values": [
//!                     "0x20",
//!                     "3",
//!                     "0x6162630000000000000000000000000000000000000000000000000000000000"
//!                 ]
//!             },
//!             {
//!                 "topics": [
//!                     "0xff00000000000000000000000000000000000000000000000000000000000000",
//!                     "0x3a56b02b60d4990074262f496ac34733f870e1b7815719b46ce155beac5e1a41"
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
    event Empty(uint256 indexed a);
    event OneWord(uint200 indexed i, int8 indexed, uint256 x) anonymous;
    event TwoWords(Str indexed, Str);
    event ThreeWords(string indexed, Enum indexed ei, Str, Enum e) anonymous;
    event Dynamic(bytes2 indexed, string);
    event Complex(bytes1 indexed b, bytes indexed bd, Str s, bytes, uint256[1], Enum[] e) anonymous;

    function test(uint256 number) public {
        emit Empty(8923892323892398);
        emit OneWord(2**200-1, -128, CONST);
        emit TwoWords(Str(1, 1), Str(-128, 2**256-1));
        emit ThreeWords("some string", Enum.B, Str(127, number), Enum.C);
        emit Dynamic(bytes2(0xabcd), "abc");
        Enum[] memory enums = new Enum[](3);
        enums[0] = Enum.A;
        enums[1] = Enum.C;
        enums[2] = Enum.B;
        emit Complex(bytes1(0xff), hex"1234567890", Str(-1, IMMUTABLE), storage_var, [uint256(0)], enums);
    }
}
