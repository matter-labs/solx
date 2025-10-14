//! { "cases": [ {
//!     "name": "empty_init_block",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "0" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "return_in_init_block",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "1" ]
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef01"
//!     ]
//! }, {
//!     "name": "revert_in_init_block",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "2" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [ "0xdeadbeef02" ],
//!         "exception": true
//!     }
//! }, {
//!     "name": "true_condition",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "3" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef03" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef03" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef03" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "false_condition",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "4" ]
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef04"
//!     ]
//! }, {
//!     "name": "string_condition",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "5" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef05" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef05" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef05" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef05" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "empty_string_condition",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "6" ]
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef06"
//!     ]
//! }, {
//!     "name": "zero_condition",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "7" ]
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef07"
//!     ]
//! }, {
//!     "name": "one_condition",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "8" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef08" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "number_condition",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "9" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef09" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef0a" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef0b" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef0c" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef0d" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "runtime_condition",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "10" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef0a" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef0a" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "return_in_condition",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "11" ]
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef0b"
//!     ]
//! }, {
//!     "name": "revert_in_condition",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "12" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!             "0xdeadbeef0c"
//!         ],
//!         "exception": true
//!     }
//! }, {
//!     "name": "empty_post_block",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "13" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef0d" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "return_in_post_block",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "14" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [ "0xdeadbeef0e" ],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef0e" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "revert_in_post_block",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "15" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [ "0xdeadbeef0f" ],
//!         "exception": true
//!     }
//! }, {
//!     "name": "unreachable_post_block",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "16" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef10" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "empty_body",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "17" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef11" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef11" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "return_in_body",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "18" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [ "0xdeadbeef12" ],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef12" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "revert_in_body",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "19" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [ "0xdeadbeef13" ],
//!         "exception": true
//!     }
//! }, {
//!     "name": "continue_in_body",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "20" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef14" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef14" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "break_in_body",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "21" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef15" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "nested_loops",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "22" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef16" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef17" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef18" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef19" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "continue_in_nested_loop",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "23" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef17" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef17" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef18" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef17" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef17" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef18" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "break_in_nested_loop",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "24" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef18" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef19" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef18" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef19" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "empty_loop_init_block_side_effect",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "25" ]
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef19"
//!     ]
//! }, {
//!     "name": "empty_loop_post_block_side_effect",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "26" ]
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef1a"
//!     ]
//! }, {
//!     "name": "empty_loop_condition_side_effect",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "27" ]
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef1b"
//!     ]
//! }, {
//!     "name": "empty_loop_body_side_effect",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "28" ]
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef1c"
//!     ]
//! }, {
//!     "name": "empty_loop",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "29" ]
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef1d"
//!     ]
//! }, {
//!     "name": "complex",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "30" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [ "0xdeadbeef03" ],
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
//!                 "values": [ "0xdeadbeef02" ]
//!             },
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
//!                 "values": [ "0xdeadbeef02" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef03" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef03" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef03" ]
//!             }
//!         ]
//!     }
//! } ] }

object "Test" {
    code {
        {
            let size := datasize("Test_deployed")
            codecopy(0, dataoffset("Test_deployed"), size)
            return(0, size)
        }
    }

    object "Test_deployed" {
        code {
            {
                switch calldataload(0)
                // empty_init_block
                case 0 {
                    let i := 0
                    for {} lt(i, 1) { i := add(i, 1) } {
                        mstore(0, 0xdeadbeef)
                        log0(0, 32)
                    }
                    return(0, 0)
                }
                // return_in_init_block
                case 1 {
                    let i := 0
                    mstore(0, 0xdeadbeef01)
                    for { return(0, 32) } lt(i, 1) { i := add(i, 1) } {
                        mstore(0, 0xe2202)
                        log0(0, 32)
                    }
                    revert(0, 32)
                }
                // revert_in_init_block
                case 2 {
                    let i := 0
                    mstore(0, 0xdeadbeef02)
                    for { revert(0, 32) } lt(i, 1) { i := add(i, 1) } {
                        mstore(0, 0xe2202)
                        log0(0, 32)
                    }
                    return(0, 32)
                }
                // true_condition
                case 3 {
                    for { let i := 0 } true { i := add(i, 1) } {
                        mstore(0, 0xdeadbeef03)
                        log0(0, 32)
                        if eq(i, 2) {
                            break
                        }
                    }
                    return(0, 0)
                }
                // false_condition
                case 4 {
                    mstore(0, 0xdeadbeef04)
                    for { let i := 0 } false { i := add(i, 1) } {
                        mstore(0, 0xe2202)
                        log0(0, 32)
                    }
                    return(0, 32)
                }
                // string_condition
                case 5 {
                    for { let i := 0 } "abc" { i := add(i, 1) } {
                        mstore(0, 0xdeadbeef05)
                        log0(0, 32)
                        if eq(i, 3) {
                            break
                        }
                    }
                    return(0, 0)
                }
                // empty_string_condition
                case 6 {
                    mstore(0, 0xdeadbeef06)
                    for { let i := 0 } "" { i := add(i, 1) } {
                        mstore(0, 0xe2202)
                        log0(0, 32)
                    }
                    return(0, 32)
                }
                // zero_condition
                case 7 {
                    mstore(0, 0xdeadbeef07)
                    for { let i := 0 } 0 { i := add(i, 1) } {
                        mstore(0, 0xe2202)
                        log0(0, 32)
                    }
                    return(0, 32)
                }
                // one_condition
                case 8 {
                    for { let i := 0 } 1 { i := add(i, 1) } {
                        mstore(0, 0xdeadbeef08)
                        log0(0, 32)
                        if eq(i, 0) {
                            break
                        }
                    }
                    return(0, 0)
                }
                // number_condition
                case 9 {
                    for { let i := 0 } 344342323832832883 { i := add(i, 1) } {
                        mstore(0, add(0xdeadbeef09, i))
                        log0(0, 32)
                        if eq(i, 4) {
                            break
                        }
                    }
                    return(0, 0)
                }
                // runtime_condition
                case 10 {
                    for { let i := 0 } lt(add(callvalue(), i), 2) { i := add(i, 1) } {
                        mstore(0, 0xdeadbeef0a)
                        log0(0, 32)
                    }
                    return(0, 0)
                }
                // return_in_condition
                case 11 {
                    function f() -> ret {
                        mstore(0, 0xdeadbeef0b)
                        ret := 1
                        return(0, 32)
                    }
                    for { let i := 0 } f() { i := add(i, 1) } {
                        mstore(0, 0xe2202)
                        log0(0, 32)
                    }
                    revert(0, 32)
                }
                // revert_in_condition
                case 12 {
                    function f() -> ret {
                        mstore(0, 0xdeadbeef0c)
                        revert(0, 32)
                    }
                    for { let i := 0 } f() { i := add(i, 1) } {
                        mstore(0, 0xe2202)
                        log0(0, 32)
                    }
                    return(0, 32)
                }
                // empty_post_block
                case 13 {
                    for { let i := 0 } lt(i, 1) {} {
                        mstore(0, 0xdeadbeef0d)
                        log0(0, 32)
                        i := add(i, 1)
                    }
                    return(0, 0)
                }
                // return_in_post_block
                case 14 {
                    for { let i := 0 } true { mstore(0, 0xdeadbeef0e) return(0, 32) } {
                        mstore(0, 0xdeadbeef0e)
                        log0(0, 32)
                    }
                    revert(0, 32)
                }
                // revert_in_post_block
                case 15 {
                    for { let i := 0 } lt(i, 23232) { mstore(0, 0xdeadbeef0f) revert(0, 32) } {
                        mstore(0, 0xe2202)
                        log0(0, 32)
                    }
                    return(0, 32)
                }
                // unreachable_post_block
                case 16 {
                    for { let i := 0 } lt(i, 23232) { revert(0, 32) } {
                        mstore(0, 0xdeadbeef10)
                        log0(0, 32)
                        break
                    }
                    return(0, 0)
                }
                // empty_body
                case 17 {
                    for { let i := 0 } lt(i, 2) { mstore(0, 0xdeadbeef11) log0(0, 32) i := add(i, 1) } {}
                    return(0, 0)
                }
                // return_in_body
                case 18 {
                    for { let i := 0 } iszero(eq(i, 5)) { i := add(i, 1) } {
                        mstore(0, 0xdeadbeef12)
                        log0(0, 32)
                        return(0, 32)
                        mstore(0, 0xe2202)
                        log0(0, 32)
                    }
                    revert(0, 32)
                }
                // revert_in_body
                case 19 {
                    for { let i := 0 } iszero(eq(i, 5)) { i := add(i, 1) } {
                        mstore(0, 0xdeadbeef13)
                        revert(0, 32)
                        mstore(0, 0xe2202)
                        log0(0, 32)
                    }
                    return(0, 32)
                }
                // continue_in_body
                case 20 {
                    for { let i := 0 } iszero(eq(i, 2)) { i := add(i, 1) } {
                        mstore(0, 0xdeadbeef14)
                        log0(0, 32)
                        continue
                        revert(0, 32)
                    }
                    return(0, 0)
                }
                // break_in_body
                case 21 {
                    for { let i := 5 } gt(i, 3) { i := sub(i, 1) } {
                        mstore(0, 0xdeadbeef15)
                        log0(0, 32)
                        break
                        mstore(0, 0xe2202)
                        log0(0, 32)
                    }
                    return(0, 0)
                }
                // nested_loops
                case 22 {
                    for { let i := 0 } lt(i, 2) { i := add(i, 1) } {
                        for { let j := 0 } lt(j, 2) { j := add(j, 1) } {
                            mstore(0, add(0xdeadbeef16, add(mul(i, 2), j)))
                            log0(0, 32)
                        }
                    }
                    return(0, 0)
                }
                // continue_in_nested_loop
                case 23 {
                    for { let i := 0 } lt(i, 2) { i := add(i, 1) } {
                        for { let j := 0 } lt(j, 2) { j := add(j, 1) } {
                            mstore(0, 0xdeadbeef17)
                            log0(0, 32)
                            continue
                            mstore(0, 0xe2202)
                            log0(0, 32)
                        }
                        mstore(0, 0xdeadbeef18)
                        log0(0, 32)
                    }
                    return(0, 0)
                }
                // break_in_nested_loop
                case 24 {
                    for { let i := 0 } lt(i, 2) { i := add(i, 1) } {
                        for { let j := 0 } lt(j, 2) { j := add(j, 1) } {
                            mstore(0, 0xdeadbeef18)
                            log0(0, 32)
                            break
                            mstore(0, 0xe2202)
                            log0(0, 32)
                        }
                        mstore(0, 0xdeadbeef19)
                        log0(0, 32)
                    }
                    return(0, 0)
                }
                // empty_loop_init_block_side_effect
                case 25 {
                    for { mstore(0, 0xdeadbeef19) } false {} {}
                    return(0, 32)
                }
                // empty_loop_post_block_side_effect
                case 26 {
                    mstore(0, 0xdeadbeef1a)
                    for {} true { return(0, 32) } {}
                    revert(0, 0)
                }
                // empty_loop_condition_side_effect
                case 27 {
                    function f() -> ret {
                        mstore(0, 0xdeadbeef1b)
                    }
                    for {} f() {} {}
                    return(0, 32)
                }
                // empty_loop_body_side_effect
                case 28 {
                    mstore(0, 0xdeadbeef1c)
                    for {} true {} {
                        return(0, 32)
                    }
                    revert(0, 0)
                }
                // empty_loop
                case 29 {
                    mstore(0, 0xdeadbeef1d)
                    for {} false {} {}
                    return(0, 32)
                }
                // complex
                case 30 {
                    for {} true {} {
                        for { let i := 0 } lt(i, 2) { i := add(i, 1) } {
                            for { let p := callvalue() } lt(p, 2) { p := add(p, 1) } {
                                if eq(p, 1) {
                                    continue
                                }
                                mstore(0, 0xdeadbeef01)
                                log0(0, 32) // 1 * 2 * 1 = 2 times
                            }
                            for { let j := 2 } gt(j, 0) { j := sub(j, 1) } {
                                for {} true {} {
                                    mstore(0, 0xdeadbeef02)
                                    log0(0, 32) // 1 * 2 * 2 * 1 = 4 times
                                    break
                                    revert(0, 32)
                                }
                                if eq(j, 1) {
                                    break
                                }
                            }
                        }
                        for { let h := 1 } lt(h, 5) { h := add(h, 1) } {
                            mstore(0, 0xdeadbeef03)
                            log0(0, 32) // 1 * 3 = 3 times
                            if eq(h, 3) {
                                return(0, 32)
                            }
                        }
                        break
                    }
                    revert(0, 0)
                }
            }
        }
    }
}