
//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "f",
//!             "calldata": [
//!             ]
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

pragma solidity >=0.4.22;

contract C {

  function() internal storedFn;

  bool flag;

  constructor() public {
    if (!flag) {
      flag = true;
      function() internal invalid;
      storedFn = invalid;
      invalid();
    }
  }
  function f() public pure {}
}
contract Test {
  function f() public {
    new C();
  }
}
// ====
// compileToEwasm: also
// ----
// f() -> FAILURE, hex"4e487b71", 0x51
