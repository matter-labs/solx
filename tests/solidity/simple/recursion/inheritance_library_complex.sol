//! { "modes": [ "Y", "E" ], "cases": [ {
//!     "name": "f",
//!     "inputs": [
//!         {
//!             "method": "p0",
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

library Foo {
  function x(uint a) internal returns (uint) {
    if (a == 0)
      return x(a);
    return 1;
  }
  function y(uint a) internal returns (uint) { return 2; }
  function z(uint a) internal returns (uint) {
    if (a == 0)
      return x(a);
    return 2;
  }

  function p(function(uint) internal returns (uint) a) internal returns (uint) {
    return a(1);
  }
  function f(uint a) internal returns (uint) { return a + p(x) + p(y); }
  function g(uint a) internal returns (uint) { return p(z); }
}

library Bar {
  function x(uint a) internal returns (uint) {
    if (a == 0)
      return x(a);
    return 1;
  }
  function p() internal returns (function(uint) internal returns (uint) a) {
    return x;
  }
  function f(uint a) internal returns (uint) { return a + p()(a); }
}

contract P2 {
  using Bar for uint;
  function p2(uint a) public returns (uint) {
    return Foo.f(a) + a.f();
  }
}

contract P1 is P2 {
  using Bar for uint;
  function p1(uint a) public returns (uint) {
    return a.f();
  }
}

contract P0 is P1 {
  function p0(uint a) public returns (uint) {
    return Foo.f(a) + p1(a) + p2(a);
  }
}

contract Test is P0 {}
