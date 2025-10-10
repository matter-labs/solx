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
//!                     "0x448a4dbfbb0cb14af554ed9a5cbdaf4ecc08c9a9db0565418ce03921991585e9",
//!                     "8923892323892398",
//!                     "0x58ae9129f801e721b1bbf220c77ba8c48badd9b887f33def390a8120ed73c9cc",
//!                     "0x0f551aaa5c86ee8370a062ef34113b9589b1b5d8c4fd999217b22b5f6e4dc726"
//!                 ],
//!                 "values": []
//!             },
//!             {
//!                 "topics": [
//!                     "0xffffffffffffffffffffffffffffffffffffffffffffffffff",
//!                     "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80",
//!                     "0x000000000000000000000000000000000000000000000000000000ffffffffff",
//!                     "0xa9c584056064687e149968cbab758a3376d22aedc6a55823d1b3ecbee81b8fb9"
//!                 ],
//!                 "values": ["0xdeadbeef"]
//!             },
//!             {
//!                 "topics": [
//!                     "0x251115a7025a1da3bd9ba76e9286175a072dd3af7ff77ed356b054cf2a6f445a",
//!                     "0xcc69885fda6bcc1a4ace058b4a62bf5e179ea78fd58a1ccd71c22cc9b688792f",
//!                     "-1",
//!                     "0x290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563"
//!                 ],
//!                 "values": ["-128", "-1"]
//!             },
//!             {
//!                 "topics": [
//!                     "0x83c737ad570e9f3e71e0d2800958e44770d812e92db2c1758626613d1e6ba514",
//!                     "1",
//!                     "0xdf0e45714bf256afe7c550b1150d0958439b34ae307888e184b747e3dd414a4a",
//!                     "0x46fff8a91b2a509b0003c20c325d2116ddb2f88f4c48fdebba3bfce2ed90bb2c"
//!                 ],
//!                 "values": [
//!                     "127",
//!                     "0xdeadbeef2",
//!                     "2"
//!                 ]
//!             },
//!             {
//!                 "topics": [
//!                     "0xdca3c296240fe7b6419be9ee836b72c9aa783fa278021c0959f7c8d1c176ae60",
//!                     "0xabcd000000000000000000000000000000000000000000000000000000000000",
//!                     "0xdeadbeef00000000000000000000000000000000000000000000000000000000",
//!                     "0x45030b51678974217968eeacdbef5b0ac42a4c6351c961a8a80591f1b70c14cc"
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
//!                     "0x5bb661834c674f8f35aa5071b921a42aeae177c4b8bec3e41e2beb11cc404805",
//!                     "Test.address"
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
    event Empty(uint256 indexed a, uint8[2] indexed, Str[] indexed dyn_str_ind);
    event OneWord(uint200 indexed i, int8 indexed, uint256 x, uint40 indexed u40ind, uint256[1] indexed uint_arr1_ind) anonymous;
    event TwoWords(Str indexed, int256 indexed, Enum[1] indexed, Str);
    event ThreeWords(string indexed, Enum indexed ei, int200[] indexed dyn_arr_ind, Str, Enum e, Str[2] indexed) anonymous;
    event Dynamic(bytes2 indexed, bytes32 indexed, uint8[2][2] indexed, string);
    event Complex(bytes1 indexed b, bytes indexed bd, bytes[] indexed, address indexed addr, Str s, bytes, uint256[1], Enum[] e) anonymous;

    function test(uint256 number) public {
        Str[] memory strs = new Str[](2);
        strs[0] = Str(-1, 1);
        strs[1] = Str(127, 123456789012345678901234567890);
        emit Empty(8923892323892398, [255, 0], strs);
        emit OneWord(2**200-1, -128, CONST, 2**40-1, [2**256-1]);
        emit TwoWords(Str(1, 1), -1, [Enum.A], Str(-128, 2**256-1));
        int200[] memory ints = new int200[](3);
        ints[0] = -1;
        ints[1] = 131231231232134;
        ints[2] = 1;
        emit ThreeWords("some string", Enum.B, ints, Str(127, number), Enum.C, [Str(0, 0), Str(12, 6789983984983434348989348934934)]);
        emit Dynamic(bytes2(0xabcd), hex"deadbeef", [[4, 3], [2, 1]], "abc");
        Enum[] memory enums = new Enum[](3);
        enums[0] = Enum.A;
        enums[1] = Enum.C;
        enums[2] = Enum.B;
        bytes[] memory bytes_array = new bytes[](2);
        bytes_array[0] = hex"1234567890123456789012345678901234567890123456789012345678901234567890";
        bytes_array[1] = hex"0987654321";
        emit Complex(bytes1(0xff), hex"1234567890", bytes_array, address(this), Str(-1, IMMUTABLE), storage_var, [uint256(0)], enums);
    }
}
