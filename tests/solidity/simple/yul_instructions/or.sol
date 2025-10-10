//! { "cases": [ {
//!     "name": "zero_zero",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "0", "0"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "zero_one",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "0", "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! }, {
//!     "name": "one_zero",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "1", "0"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! }, {
//!     "name": "one_one",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "1", "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! }, {
//!     "name": "zero_ordinar",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "0", "433478394034343"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "433478394034343"
//!     ]
//! }, {
//!     "name": "ordinar_zero",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "121563127839120", "0"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "121563127839120"
//!     ]
//! }, {
//!     "name": "one_ordinar",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "1", "433478394034343"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "433478394034343"
//!     ]
//! }, {
//!     "name": "ordinar_one",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "121563127839120", "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "121563127839121"
//!     ]
//! }, {
//!     "name": "ordinar_ordinar",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "0xE0281C82903B3293902E82845FF823834309A8F239982392328448AB42789DA8", "0xA80185C2323AA131233FED325FF823834336742387428823823238FCAD223AB4"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0xe8299dc2b23bb3b3b33fefb65ff82383433ffcf3bfdaabb3b2b678ffef7abfbc"
//!     ]
//! }, {
//!     "name": "ordinar_ordinar_equals",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "0x67AC08676F781278AA823890BC392384382FE19082382834A8237678CC128912", "0x67AC08676F781278AA823890BC392384382FE19082382834A8237678CC128912"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0x67AC08676F781278AA823890BC392384382FE19082382834A8237678CC128912"
//!     ]
//! }, {
//!     "name": "ordinar_ordinar_to_max",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "0xfafffffffffffffffffffffffffffffffffffffffffffffffffffffff4ffffff", "0x050000000000000000000000000000000000000000000000000000000b000000"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
//!     ]
//! }, {
//!     "name": "zero_max",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "0", "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
//!     ]
//! }, {
//!     "name": "max_zero",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", "0"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
//!     ]
//! }, {
//!     "name": "one_max",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "1", "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
//!     ]
//! }, {
//!     "name": "max_one",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
//!     ]
//! }, {
//!     "name": "ordinar_max",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "7437834752357434334343423343443375834785783474", "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
//!     ]
//! }, {
//!     "name": "max_ordinar",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", "23784273472384723848213821342323233223"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
//!     ]
//! }, {
//!     "name": "max_max",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function main(uint256 a, uint256 b) external pure returns(uint256 result) {
        assembly {
            result := or(a, b)
        }
    }
}
