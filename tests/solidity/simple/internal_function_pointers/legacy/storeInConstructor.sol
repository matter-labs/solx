//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "f",
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

  bool flag;

  function Test() public {
    function() internal invalid;
    storedFn = invalid;
  }
  function f() public returns (uint) {
    if (flag) return 2;
    flag = true;
    storedFn();
  }
}
// ====
// compileToEwasm: also
// ----
// f() -> FAILURE, hex"4e487b71", 0x51
// f() -> FAILURE, hex"4e487b71", 0x51
