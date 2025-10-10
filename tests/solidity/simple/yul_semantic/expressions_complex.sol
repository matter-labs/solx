//! { "modes": [ "Y", "E" ], "cases": [ {
//!     "name": "complex",
//!     "inputs": [
//!         {
//!             "method": "complex",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!             "120"
//!         ],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xbbb" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xaaa" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "100" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "1500000" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "8888" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "9999999" ]
//!             }
//!         ]
//!     }
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.7.1;

contract Test {
    function complex() external {
        assembly {
            function emit(a, b) -> ret {
                mstore(0, a)
                log0(0, 32)
                ret := b
            }
            function fact(n) -> ret {
                if eq(n, 0) {
                    ret := 1
                    leave
                }
                ret := mul(
                    fact(sub(n, 1)),
                    n
                )
            }
            let a := 100
            mstore(0, fact(
                sub(
                    add(emit(9999999, 3), emit(8888, 8)), // 3 + 8 = 11
                    emit(
                        mul(
                            a,
                            mul(
                                emit(a, 5), // 5
                                add(emit(0xaaa, 1000), emit(0xbbb, 2000)) //  1000 + 2000 = 3000
                            ) // 5 * 3000 = 15000
                        ), // 100 * 15000 = 1500000
                        fact(3) // 6
                    ) // 6
                ) // 11 - 6 = 5
            )) // 5! = 120
            return(0, 32)
        }
    }
}
