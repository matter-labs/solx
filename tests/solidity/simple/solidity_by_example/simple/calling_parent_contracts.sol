//! { "cases": [ {
//!     "name": "foo",
//!     "inputs": [
//!         {
//!             "method": "foo",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!         ],
//!         "events": [
//!             {
//!                 "topics": [
//!                     "0xcf34ef537ac33ee1ac626ca1587a0a7e8e51561e5514f8cb36afa1c5102b3bab"
//!                 ],
//!                 "values": [
//!                     "0x20",
//!                     "12",
//!                     "0x432e666f6f2063616c6c65640000000000000000000000000000000000000000"
//!                 ]
//!             }, {
//!                 "topics": [
//!                     "0xcf34ef537ac33ee1ac626ca1587a0a7e8e51561e5514f8cb36afa1c5102b3bab"
//!                 ],
//!                 "values": [
//!                     "0x20",
//!                     "12",
//!                     "0x412e666f6f2063616c6c65640000000000000000000000000000000000000000"
//!                 ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "bar",
//!     "inputs": [
//!         {
//!             "method": "bar",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!         ],
//!         "events": [
//!             {
//!                 "topics": [
//!                     "0xcf34ef537ac33ee1ac626ca1587a0a7e8e51561e5514f8cb36afa1c5102b3bab"
//!                 ],
//!                 "values": [
//!                     "0x20",
//!                     "12",
//!                     "0x432e6261722063616c6c65640000000000000000000000000000000000000000"
//!                 ]
//!             }, {
//!                 "topics": [
//!                     "0xcf34ef537ac33ee1ac626ca1587a0a7e8e51561e5514f8cb36afa1c5102b3bab"
//!                 ],
//!                 "values": [
//!                     "0x20",
//!                     "12",
//!                     "0x422e6261722063616c6c65640000000000000000000000000000000000000000"
//!                 ]
//!             }, {
//!                 "topics": [
//!                     "0xcf34ef537ac33ee1ac626ca1587a0a7e8e51561e5514f8cb36afa1c5102b3bab"
//!                 ],
//!                 "values": [
//!                     "0x20",
//!                     "12",
//!                     "0x412e6261722063616c6c65640000000000000000000000000000000000000000"
//!                 ]
//!             }
//!         ]
//!     }
//! } ] }

// SPDX-License-Identifier: MIT
pragma solidity >=0.6.0;

/* Inheritance tree
   A
 /  \
B   C
 \ /
  D
*/

contract A {
    // This is called an event. You can emit events from your function
    // and they are logged into the transaction log.
    // In our case, this will be useful for tracing function calls.
    event Log(string message);

    function foo() public virtual {
        emit Log("A.foo called");
    }

    function bar() public virtual {
        emit Log("A.bar called");
    }
}

contract B is A {
    function foo() public virtual override {
        emit Log("B.foo called");
        A.foo();
    }

    function bar() public virtual override {
        emit Log("B.bar called");
        super.bar();
    }
}

contract C is A {
    function foo() public virtual override {
        emit Log("C.foo called");
        A.foo();
    }

    function bar() public virtual override {
        emit Log("C.bar called");
        super.bar();
    }
}

contract Test is B, C {
    // Try:
    // - Call D.foo and check the transaction logs.
    //   Although D inherits A, B and C, it only called C and then A.
    // - Call D.bar and check the transaction logs
    //   D called C, then B, and finally A.
    //   Although super was called twice (by B and C) it only called A once.

    function foo() public override(B, C) {
        super.foo();
    }

    function bar() public override(B, C) {
        super.bar();
    }
}
