//! { "cases": [ {
//!     "name": "encrypt",
//!     "inputs": [
//!         {
//!             "method": "encrypt",
//!             "calldata": [
//!                 "12", "143", "213", "43", "85", "42", "98", "111", "224", "76", "117"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "129", "4", "74", "160", "202", "159", "215", "228", "85", "193"
//!     ]
//! }, {
//!     "name": "decrypt",
//!     "inputs": [
//!         {
//!             "method": "decrypt",
//!             "calldata": [
//!                 "129", "4", "74", "160", "202", "159", "215", "228", "85", "193", "117"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "12", "143", "213", "43", "85", "42", "98", "111", "224", "76"
//!     ]
//! }, {
//!     "name": "complex",
//!     "inputs": [
//!         {
//!             "method": "complex",
//!             "calldata": [
//!                 "1", "12", "55", "53", "213", "123", "45", "21", "12", "92"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.19;

contract Test {
    uint8 constant SIZE = 10;

    function encrypt(uint8[SIZE] memory data, uint8 key) public pure returns(uint8[SIZE] memory) {
        for(uint8 i = 0; i < SIZE; i++) {
            data[i] = uint8((uint16(data[i]) + key) % 256);
        }
        return data;
    }

    function decrypt(uint8[SIZE] memory data, uint8 key) public pure returns(uint8[SIZE] memory) {
        return encrypt(data, uint8((256 - uint16(key)) % 256));
    }

    function complex(uint8[SIZE] memory message) public pure returns(uint64) {
        bool result = true;
        for(uint8 key = 0; key < 20; key++) {
            uint8[SIZE] memory output = decrypt(encrypt(message, key), key);
            for(uint8 i = 0; i < SIZE; i++) {
                result = result && message[i] == output[i];
            }
        }
        if (result) {
            return 1;
        } else {
            return 0;
        }
    }
}