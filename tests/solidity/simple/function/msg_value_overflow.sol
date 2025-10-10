//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "entry",
//!             "calldata": [
//!                 "0x100000000000000000000000000000001"
//!             ],
//!             "value": "340282366920938463463374607431768211455 wei"
//!         }
//!     ],
//!     "expected": {
//!         "exception": true,
//!         "return_data": []
//!     }
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.6.2;

contract Test {
    function entry(uint256 value) external payable returns(uint256) {
        return this.main{value: value}();
    }

    function main() external payable returns(uint256) {
        return msg.value;
    }
}
