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
//!         "0", "1", "0", "0"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.7.0;

contract Test {
    uint256 public t1;
    uint256 public c1;
    uint256 public t2;
    uint256 public c2;

    function main() public returns (uint256, uint256, uint256, uint256) {
        try this.f1{gas: 50000}() {
            t1++;
        } catch {
            c1++;
        }
        return (t1, c1, t2, c2);
    }

    function f1() public {
        try this.f2{gas: 500000}() {
            t2++;
        } catch {
            c2++;
        }
    }

    function f2() public {
        assembly {
            return(0, 320000000000)
        }
    }
}
