//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.6.0;

interface ITest2 {
    function transfer(address to, uint256 balance) external;
}

contract Test {
    Test2 ins;

    constructor() public {
        ins = new Test2();
    }

    function main() public returns (bool) {
        try ITest2(address(ins)).transfer(msg.sender, 0) {
            return true;
        } catch {
            return false;
        }
    }
}

contract Test2 {
    function transfer(
        bytes4[9] memory,
        bytes5[6] memory,
        int48[11] memory
    ) public {}
}
