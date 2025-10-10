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
//!                     "0x2013d4ad9a9a6379fb85265fd28e4d4e2f3d4b6a9087d5a1fa3aed3548ecada3",
//!                     "8923892323892398",
//!                     "0x58ae9129f801e721b1bbf220c77ba8c48badd9b887f33def390a8120ed73c9cc"
//!                 ],
//!                 "values": []
//!             },
//!             {
//!                 "topics": [
//!                     "0xffffffffffffffffffffffffffffffffffffffffffffffffff",
//!                     "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80",
//!                     "0x000000000000000000000000000000000000000000000000000000ffffffffff"
//!                 ],
//!                 "values": ["0xdeadbeef"]
//!             },
//!             {
//!                 "topics": [
//!                     "0x9ce9673b07118eb7e55c7fc521b8bf9d6a25c247b2c90cf0208699e1ee393ad6",
//!                     "0xcc69885fda6bcc1a4ace058b4a62bf5e179ea78fd58a1ccd71c22cc9b688792f",
//!                     "-1"
//!                 ],
//!                 "values": ["-128", "-1"]
//!             },
//!             {
//!                 "topics": [
//!                     "0x83c737ad570e9f3e71e0d2800958e44770d812e92db2c1758626613d1e6ba514",
//!                     "1",
//!                     "0xdf0e45714bf256afe7c550b1150d0958439b34ae307888e184b747e3dd414a4a"
//!                 ],
//!                 "values": [
//!                     "127",
//!                     "0xdeadbeef2",
//!                     "2"
//!                 ]
//!             },
//!             {
//!                 "topics": [
//!                     "0xf20f5ffcb271503e510c980b74d44f6628d621d291d150a564d6e1e261862374",
//!                     "0xabcd000000000000000000000000000000000000000000000000000000000000",
//!                     "0xdeadbeef00000000000000000000000000000000000000000000000000000000"
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
//!                     "0x3a56b02b60d4990074262f496ac34733f870e1b7815719b46ce155beac5e1a41",
//!                     "0x5bb661834c674f8f35aa5071b921a42aeae177c4b8bec3e41e2beb11cc404805"
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
    event Empty(uint256 indexed a, uint8[2] indexed);
    event OneWord(uint200 indexed i, int8 indexed, uint256 x, uint40 indexed u40ind) anonymous;
    event TwoWords(Str indexed, int256 indexed, Str);
    event ThreeWords(string indexed, Enum indexed ei, int200[] indexed dyn_arr_ind, Str, Enum e) anonymous;
    event Dynamic(bytes2 indexed, bytes32 indexed, string);
    event Complex(bytes1 indexed b, bytes indexed bd, bytes[] indexed, Str s, bytes, uint256[1], Enum[] e) anonymous;

    function test(uint256 number) public {
        emit Empty(8923892323892398, [255, 0]);
        emit OneWord(2**200-1, -128, CONST, 2**40-1);
        emit TwoWords(Str(1, 1), -1, Str(-128, 2**256-1));
        int200[] memory ints = new int200[](3);
        ints[0] = -1;
        ints[1] = 131231231232134;
        ints[2] = 1;
        emit ThreeWords("some string", Enum.B, ints, Str(127, number), Enum.C);
        emit Dynamic(bytes2(0xabcd), hex"deadbeef", "abc");
        Enum[] memory enums = new Enum[](3);
        enums[0] = Enum.A;
        enums[1] = Enum.C;
        enums[2] = Enum.B;
        bytes[] memory bytes_array = new bytes[](2);
        bytes_array[0] = hex"1234567890123456789012345678901234567890123456789012345678901234567890";
        bytes_array[1] = hex"0987654321";
        emit Complex(bytes1(0xff), hex"1234567890", bytes_array, Str(-1, IMMUTABLE), storage_var, [uint256(0)], enums);
    }
}
