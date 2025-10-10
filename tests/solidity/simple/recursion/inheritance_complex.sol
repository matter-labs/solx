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
//!         "11"
//!     ]
//! }, {
//!     "name": "h",
//!     "inputs": [
//!         {
//!             "method": "h",
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

pragma solidity >=0.4.12;

contract P2 {
  function h(uint a) public returns (uint) {
    if (a == 1)
      return 12;
    return h(a);
  }
}
contract P1 is P2 {
  function g(uint a) public returns (uint) {
    if (a == 1)
      return 11;
    return g(a);
  }
}
contract P0 is P1 {
  function f(uint a) public returns (uint) {
    if (a == 1)
      return 10;
    return f(a);
  }
}
contract Test is P0 {}
