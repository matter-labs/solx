//! { "cases": [ {
//!     "name": "complex1",
//!     "inputs": [
//!         {
//!             "method": "complex",
//!             "calldata": [
//!                 "1", "12", "55", "53", "22", "34", "45", "21", "12", "7"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1", "12", "55", "53", "22", "34", "45", "21", "12", "7"
//!     ]
//! }, {
//!     "name": "complex2",
//!     "inputs": [
//!         {
//!             "method": "complex",
//!             "calldata": [
//!                 "37", "11", "2", "59", "0", "63", "41", "27", "17", "9"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "37", "11", "2", "59", "0", "63", "41", "27", "17", "9"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.19;

contract Test {
    uint8 constant KEY_SZ = 4;
    uint8[KEY_SZ][KEY_SZ][KEY_SZ] KEY =
    [
        [
            [10, 33, 26, 15],
            [32, 62, 43, 58],
            [3, 8, 45, 23],
            [13, 55, 30, 21]
        ], [
            [11, 22, 9, 2],
            [28, 52, 38, 27],
            [61, 53, 56, 37],
            [5, 25, 39, 59]
        ], [
            [19, 36, 4, 48],
            [51, 0, 44, 35],
            [54, 46, 6, 60],
            [14, 31, 63, 29]
        ], [
            [40, 7, 20, 50],
            [41, 12, 49, 16],
            [18, 17, 57, 34],
            [42, 24, 1, 47]
        ]
    ];


    uint8 constant SIZE = 10;
    uint8 constant ALPHABET = 64;

    function encrypt(uint8[SIZE] memory data) private view returns(uint8[SIZE] memory) {
        for(uint8 i = 0; i < SIZE; i++) {
            bool fl = false;
            for(uint8 page = 0; page < KEY_SZ; page++) {
                for(uint8 row = 0; row < KEY_SZ; row++) {
                    for(uint8 col = 0; col < KEY_SZ; col++) {
                        if (KEY[page][row][col] == data[i]) {
                            data[i] = page*KEY_SZ*KEY_SZ + row*KEY_SZ + col;
                            fl = true;
                            break;
                        }
                    }
                    if (fl) {
                        break;
                    }
                }
                if (fl) {
                    break;
                }
            }
        }
        return data;
    }

    function decrypt(uint8[SIZE] memory data) private view returns(uint8[SIZE] memory) {
        for(uint8 i = 0; i < SIZE; i++) {
            uint8 page = data[i] / KEY_SZ / KEY_SZ;
            uint8 ost = data[i] % (KEY_SZ * KEY_SZ);
            uint8 row = ost / KEY_SZ;
            uint8 col = ost % KEY_SZ;
            data[i] = KEY[page][row][col];
        }
        return data;
    }

    function complex(uint8[SIZE] memory message) public view returns(uint8[SIZE] memory) {
        return decrypt(encrypt(message));
    }
}