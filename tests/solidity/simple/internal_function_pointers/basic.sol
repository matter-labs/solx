//! { "modes": [ "Y", "E" ], "cases": [ {
//!     "name": "success",
//!     "inputs": [
//!         {
//!             "method": "#deployer",
//!             "calldata": [
//!                 "1"
//!             ],
//!             "expected": [
//!                 "Test.address"
//!             ]
//!         },
//!         {
//!             "method": "m",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "386"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.22;

contract Test {
    uint x;

    function() internal pure returns (uint) fp;
    function c0() internal pure returns (uint) { return 0xc0; }
    function c1() internal pure returns (uint) { return 0xc1; }

    constructor(int i) public {
        if (i == 0) { fp = c0; }
        if (i == 1) { fp = c1; }
        x = fp();
    }

    function m() public view returns (uint) {
        return x + fp();
    }
}
