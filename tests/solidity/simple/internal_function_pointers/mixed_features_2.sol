//! { "modes": [ "Y >=0.8.1", "E" ], "cases": [ {
//!     "name": "process",
//!     "inputs": [
//!         {
//!             "method": "process",
//!             "calldata": [ "16", "3" ]
//!         }
//!     ],
//!     "expected": [ "250" ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.6.0;

contract OtherContract {
    function other(uint256 input) external pure returns(uint256) {
        return input * 5;
    }
}

contract Test {
    struct MyStruct {
        uint256[1] arr;
        function (uint256) external pure returns(uint256) func;
    }

    OtherContract other;

    constructor() public {
        other = new OtherContract();
    }

    modifier onlyEven(uint256 val) {
        require(val % 2 == 0, "Only even numbers allowed");
        _;
    }

    function process(uint256 val, uint256 depth) onlyEven(val) public returns(uint256) {
        if (depth == 0) {
            return val;
        }

        MyStruct memory myStruct;
        myStruct.arr[0] = val / 2;
        myStruct.func = other.other;

        uint256 newVal = myStruct.func(myStruct.arr[0]);

        return process(newVal, depth - 1);
    }
}
