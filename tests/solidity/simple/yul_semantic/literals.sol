//! { "cases": [ {
//!     "name": "decimal",
//!     "inputs": [
//!         {
//!             "method": "decimal",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!         "4024342343240230"
//!     ]
//! }, {
//!     "name": "hexadecimal",
//!     "inputs": [
//!         {
//!             "method": "hexadecimal",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef"
//!     ]
//! }, {
//!     "name": "true",
//!     "inputs": [
//!         {
//!             "method": "_true",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! }, {
//!     "name": "false",
//!     "inputs": [
//!         {
//!             "method": "_false",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "string",
//!     "inputs": [
//!         {
//!             "method": "_string",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!         "0x6a73646a736a646a6431323331325c2f2212e2888e0000000000000000000000"
//!     ]
//! }, {
//!     "name": "hex",
//!     "inputs": [
//!         {
//!             "method": "_hex",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!         "0x0012340000000000000000000000000000000000000000000000000000000000"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.4;

contract Test {
    function decimal() external pure {
        assembly {
            mstore(0, 4024342343240230)
            return(0, 32)
        }
    }

    function hexadecimal() external pure {
        assembly {
            mstore(0, 0x00000000000deadbeef)
            return(0, 32)
        }
    }

    function _true() external pure {
        assembly {
            mstore(0, true)
            return(0, 32)
        }
    }

    function _false() external pure {
        assembly {
            mstore(0, false)
            return(0, 32)
        }
    }

    function _string() external pure {
        assembly {
            mstore(0, "jsdjsjdjd\
12312\\/\"\x12\u220E")
            return(0, 32)
        }
    }

    function _hex() external pure {
        assembly {
            mstore(0, hex"001234")
            return(0, 32)
        }
    }
}