//! { "cases": [ {
//!     "name": "literal",
//!     "inputs": [
//!         {
//!             "method": "literal",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef"
//!     ]
//! }, {
//!     "name": "variable",
//!     "inputs": [
//!         {
//!             "method": "variable",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef01"
//!     ]
//! }, {
//!     "name": "function_call",
//!     "inputs": [
//!         {
//!             "method": "function_call",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef02"
//!     ]
//! }, {
//!     "name": "nested_calls",
//!     "inputs": [
//!         {
//!             "method": "nested_calls",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef0"
//!     ]
//! }, {
//!     "name": "evaluation_order",
//!     "inputs": [
//!         {
//!             "method": "evaluation_order",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!             "0x300"
//!         ],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef01" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef02" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "nested_calls_evaluation_order",
//!     "inputs": [
//!         {
//!             "method": "nested_calls_evaluation_order",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!             "0x600"
//!         ],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef01" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef02" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef03" ]
//!             }
//!         ]
//!     }
//! } ] }


// SPDX-License-Identifier: MIT

pragma solidity >=0.4.12;

contract Test {
    function literal() external {
        assembly {
            mstore(0, 0xdeadbeef)
            return(0, 32)
        }
    }

    function variable() external {
        assembly {
            let a := 0xdeadbeef01
            mstore(0, a)
            return(0, 32)
        }
    }

    function function_call() external {
        assembly {
            function f() -> ret {
                ret := 0xdeadbeef02
            }
            mstore(0, f())
            return(0, 32)
        }
    }

    function nested_calls() external {
        assembly {
            function f1(a) -> ret {
                ret := mul(2, a)
            }
            function f2(a) -> ret {
                ret := mul(8, a)
            }
            mstore(0, f2(f1(0xdeadbeef))) // 0xdeadbeef * 2 * 8 = 16 * 0xdeadbeef = 0xdeadbeef0
            return(0, 32)
        }
    }

    function evaluation_order() external {
        assembly {
            function side_effect_1() -> ret {
                mstore(0, 0xdeadbeef01)
                log0(0, 32)
                ret := 0x100
            }
            function side_effect_2() -> ret {
                mstore(0, 0xdeadbeef02)
                log0(0, 32)
                ret := 0x200
            }
            mstore(0, add(side_effect_2(), side_effect_1())) // 0x100 + 0x200 = 0x300
            return(0, 32)
        }
    }

    function nested_calls_evaluation_order() external {
        assembly {
            function side_effect_1() -> ret {
                mstore(0, 0xdeadbeef01)
                log0(0, 32)
                ret := 0x100
            }
            function side_effect_2() -> ret {
                mstore(0, 0xdeadbeef02)
                log0(0, 32)
                ret := 0x200
            }
            function side_effect_3() -> ret {
                mstore(0, 0xdeadbeef03)
                log0(0, 32)
                ret := 0x300
            }
            function sum(a, b) -> ret {
                ret := add(a, b)
            }
            mstore(0, add(side_effect_3(), sum(side_effect_2(), side_effect_1()))) // 0x100 + 0x200 + 0x300 = 0x600
            return(0, 32)
        }
    }
}
