//! { "cases": [ {
//!     "name": "boo",
//!     "inputs": [
//!         {
//!             "method": "boo",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! }, {
//!     "name": "u8",
//!     "inputs": [
//!         {
//!             "method": "u8",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! }, {
//!     "name": "u256",
//!     "inputs": [
//!         {
//!             "method": "u256",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "456"
//!     ]
//! }, {
//!     "name": "u",
//!     "inputs": [
//!         {
//!             "method": "u()",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "123"
//!     ]
//! }, {
//!     "name": "i8",
//!     "inputs": [
//!         {
//!             "method": "i8",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "-1"
//!     ]
//! }, {
//!     "name": "i256",
//!     "inputs": [
//!         {
//!             "method": "i256",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "456"
//!     ]
//! }, {
//!     "name": "i",
//!     "inputs": [
//!         {
//!             "method": "i()",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "-123"
//!     ]
//! }, {
//!     "name": "minInt",
//!     "inputs": [
//!         {
//!             "method": "minInt",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0x8000000000000000000000000000000000000000000000000000000000000000"
//!     ]
//! }, {
//!     "name": "maxInt",
//!     "inputs": [
//!         {
//!             "method": "maxInt",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0x7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
//!     ]
//! }, {
//!     "name": "addr",
//!     "inputs": [
//!         {
//!             "method": "addr",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0xCA35b7d915458EF540aDe6068dFe2F44E8fa733c"
//!     ]
//! }, {
//!     "name": "defaultBoo",
//!     "inputs": [
//!         {
//!             "method": "defaultBoo",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "defaultUint",
//!     "inputs": [
//!         {
//!             "method": "defaultUint",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "defaultInt",
//!     "inputs": [
//!         {
//!             "method": "defaultInt",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "defaultAddr",
//!     "inputs": [
//!         {
//!             "method": "defaultAddr",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT
pragma solidity >=0.6.8;

contract Test {
    bool public boo = true;

    /*
    uint stands for unsigned integer, meaning non negative integers
    different sizes are available
        uint8   ranges from 0 to 2 ** 8 - 1
        uint16  ranges from 0 to 2 ** 16 - 1
        ...
        uint256 ranges from 0 to 2 ** 256 - 1
    */
    uint8 public u8 = 1;
    uint public u256 = 456;
    uint public u = 123; // uint is an alias for uint256

    /*
    Negative numbers are allowed for int types.
    Like uint, different ranges are available from int8 to int256

    int256 ranges from -2 ** 255 to 2 ** 255 - 1
    int128 ranges from -2 ** 127 to 2 ** 127 - 1
    */
    int8 public i8 = -1;
    int public i256 = 456;
    int public i = -123; // int is same as int256

    // minimum and maximum of int
    int public minInt = type(int).min;
    int public maxInt = type(int).max;

    address public addr = 0xCA35b7d915458EF540aDe6068dFe2F44E8fa733c;

    // Default values
    // Unassigned variables have a default value
    bool public defaultBoo; // false
    uint public defaultUint; // 0
    int public defaultInt; // 0
    address public defaultAddr; // 0x0000000000000000000000000000000000000000
}
