//! { "cases": [ {
//!     "name": "declare_without_value",
//!     "inputs": [
//!         {
//!             "method": "declare_without_value",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "declare_tuple_without_value",
//!     "inputs": [
//!         {
//!             "method": "declare_tuple_without_value",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!         "0", "0", "0"
//!     ]
//! }, {
//!     "name": "declare_with_value",
//!     "inputs": [
//!         {
//!             "method": "declare_with_value",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef"
//!     ]
//! }, {
//!     "name": "declare_tuple_with_value",
//!     "inputs": [
//!         {
//!             "method": "declare_tuple_with_value",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!         "1", "2", "3"
//!     ]
//! }, {
//!     "name": "assign",
//!     "inputs": [
//!         {
//!             "method": "assign",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef"
//!     ]
//! }, {
//!     "name": "assign_tuple",
//!     "inputs": [
//!         {
//!             "method": "assign_tuple",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!         "1", "2", "3"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.17;

contract Test {
    function declare_without_value() external {
        assembly {
            let x
            mstore(0, x)
            return(0, 32)
        }
    }

    function declare_tuple_without_value() external {
        assembly {
            let x, y, z
            mstore(0, x)
            mstore(32, y)
            mstore(64, z)
            return(0, 96)
        }
    }

    function declare_with_value() external {
        assembly {
            let x := 0xdeadbeef
            mstore(0, x)
            return(0, 32)
        }
    }

    function declare_tuple_with_value() external {
        assembly {
            function f() -> _1, _2, _3 {
                _1 := 1
                _2 := 2
                _3 := 3
            }
            let x, y, z := f()
            mstore(0, x)
            mstore(32, y)
            mstore(64, z)
            return(0, 96)
        }
    }

    function assign() external {
        assembly {
            let x
            x := 0xdeadbeef
            mstore(0, x)
            return(0, 32)
        }
    }

    function assign_tuple() external {
        assembly {
            function f() -> _1, _2, _3 {
                _1 := 1
                _2 := 2
                _3 := 3
            }
            let x, y, z
            x, y, z := f()
            mstore(0, x)
            mstore(32, y)
            mstore(64, z)
            return(0, 96)
        }
    }
}
