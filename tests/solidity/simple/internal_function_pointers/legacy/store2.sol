//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "run",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!     {
//!         "compiler_version": ">=0.8",
//!         "return_data": [
//!             "0x4e487b7100000000000000000000000000000000000000000000000000000000",
//!             "0x0000005100000000000000000000000000000000000000000000000000000000"
//!         ],
//!         "exception": true
//!     },
//!     {
//!         "compiler_version": "<=0.7",
//!         "return_data": [],
//!         "exception": true
//!     }
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity <=0.4.21;

contract Test {

  function() internal storedFn;
  uint public x;

  function Test() public {
    uint _y1;
    uint _y2;
    uint _y3;
    uint _y4;
    uint _y5;
    uint _y6;
    uint _y7;
    uint _y8;
    uint _y9;
    uint _y10;
    uint _y11;
    uint _y12;
    uint _y13;
    uint _y14;


    function() internal invalid;
    storedFn = invalid;
  }

  function run() public {
    // this did not always cause revert in the past
    storedFn();
  }

  function z() public {
      x++;
  }
}
// ====
// compileToEwasm: also
// ----
// run() -> FAILURE, hex"4e487b71", 0x51
