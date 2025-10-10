//! { "modes": [ "Y", "E" ], "cases": [ {
//!     "name": "f",
//!     "inputs": [
//!         {
//!             "method": "f",
//!             "calldata": [
//!                 "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "10"
//!     ]
//! }, {
//!     "name": "g",
//!     "inputs": [
//!         {
//!             "method": "g",
//!             "calldata": [
//!                 "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "12"
//!     ]
//!   }
//! ] }

// `virtual` and `override` were added in v0.6.0.
pragma solidity >=0.6.0;

contract P2 {
  function g(uint a) public returns (uint) {
    if (a == 1)
      return 12;
    return g(a);
  }
}
contract P1 is P2 {
  function f(uint a) public virtual returns (uint) {
    if (a == 1)
      return 101;
    return f(a);
  }
}
contract P0 is P1 {
  function f(uint a) public virtual override(P1) returns (uint) {
    if (a == 1)
      return 10;
    return f(a);
  }
}
contract Test is P0 {}
