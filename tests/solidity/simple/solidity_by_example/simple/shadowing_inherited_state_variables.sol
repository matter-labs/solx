//! { "modes": [
//!     "Y >=0.8.1",
//!     "E", "I"
//! ], "cases": [ {
//!     "name": "test",
//!     "inputs": [
//!         {
//!             "method": "#deployer",
//!             "calldata": [
//!             ],
//!             "expected": [
//!                 "Test.address"
//!             ]
//!         },
//!         {
//!             "method": "getName",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0x20",
//!         "10",
//!         "0x436f6e7472616374204300000000000000000000000000000000000000000000"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.0;

contract A {
    string public name = "Contract A";

    function getName() public view returns (string memory) {
        return name;
    }
}

// Shadowing is disallowed in Solidity 0.6
// This will not compile
// contract B is A {
//     string public name = "Contract B";
// }

contract Test is A {
    // This is the correct way to override inherited state variables.
    constructor() {
        name = "Contract C";
    }

    // C.getName returns "Contract C"
}
