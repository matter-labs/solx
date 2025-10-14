//! { "cases": [ {
//!     "name": "complex1",
//!     "inputs": [
//!         {
//!             "method": "complex",
//!             "calldata": [
//!                 "1", "231", "55", "221", "22", "34", "118", "21", "89", "7",
//!                 "12", "255", "0", "12"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1", "231", "55", "221", "22", "34", "118", "21", "89", "7"
//!     ]
//! }, {
//!     "name": "complex2",
//!     "inputs": [
//!         {
//!             "method": "complex",
//!             "calldata": [
//!                 "12", "128", "255", "0", "123", "12", "68", "192", "2", "19",
//!                 "17", "181", "64", "243"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "12", "128", "255", "0", "123", "12", "68", "192", "2", "19"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.19;

contract Test {
    function encryptSym(uint8 sym, uint8 keySym) private pure returns(uint8) {
        return uint8((uint16(sym) + keySym) % 256);
    }

    function decryptSym(uint8 sym, uint8 keySym) private pure returns(uint8) {
        uint16 symbol = sym;
        if (symbol < keySym) {
            symbol += 256;
        }
        return uint8(symbol - keySym);
    }

    uint8 constant SIZE = 10;
    uint8 constant KEY_SIZE = 4;
    
    function encrypt(uint8[SIZE] memory data, uint8[KEY_SIZE] memory key) private pure returns(uint8[SIZE] memory) {
        for(uint8 i = 0; i < SIZE; i++) {
            data[i] = encryptSym(data[i], key[i % KEY_SIZE]);
        }
        return data;
    }

    function decrypt(uint8[SIZE] memory data, uint8[KEY_SIZE] memory key) private pure returns(uint8[SIZE] memory) {
        for(uint8 i = 0; i < SIZE; i++) {
            data[i] = decryptSym(data[i], key[i % KEY_SIZE]);
        }
        return data;
    }

    function complex(uint8[SIZE] memory message, uint8[KEY_SIZE] memory key) public pure returns(uint8[SIZE] memory) {
        return decrypt(encrypt(message, key), key);
    }
}