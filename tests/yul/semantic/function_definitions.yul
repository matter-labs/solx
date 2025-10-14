//! { "cases": [ {
//!     "name": "no_args_and_ret",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [
//!                 "0"
//!             ]
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
//!     "name": "many_args",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [
//!                 "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "120"
//!     ]
//! }, {
//!     "name": "many_ret_values",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [
//!                 "2"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "78"
//!     ]
//! }, {
//!     "name": "empty_body_arg_side_effect",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [
//!                 "3"
//!             ]
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
//!     "name": "leave_in_function",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [
//!                 "4"
//!             ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef01" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "return_in_function",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [
//!                 "5"
//!             ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [ "0xdeadbeef01" ],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef01" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "revert_in_function",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [
//!                 "6"
//!             ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [ "0xdeadbeef01" ],
//!         "exception": true
//!     }
//! }, {
//!     "name": "default_ret_value",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [
//!                 "7"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "recursive_function",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [
//!                 "8"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "5050"
//!     ]
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
                // no_args_and_ret
                case 0 {
                    function $_() {
                        mstore(0, 0xdeadbeef)
                        log0(0, 32)
                    }
                    $_()
                }
                // many_args
                case 1 {
                    function f(_1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15) -> ret {
                        ret := add(
                            add(
                                add(
                                    add(_1, _2),
                                    add(_3, _4)
                                ),
                                add(
                                    add(_5, _6),
                                    add(_7, _8)
                                )
                            ),
                            add(
                                add(
                                    add(_9, _10),
                                    add(_11, _12)
                                ),
                                add(
                                    add(_13, _14),
                                    _15
                                )
                            )
                        )
                    }
                    mstore(0, f(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15))
                    return(0, 32)
                }
                // many_ret_values
                case 2 {
                    function f() -> _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12 {
                        _1 := 1
                        _2 := 2
                        _3 := 3
                        _4 := 4
                        _5 := 5
                        _6 := 6
                        _7 := 7
                        _8 := 8
                        _9 := 9
                        _10 := 10
                        _11 := 11
                        _12 := 12
                    }
                    let _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12 := f()
                    mstore(
                        0,
                        add(
                            add(
                                add(
                                    add(_1, _2),
                                    add(_3, _4)
                                ),
                                add(
                                    add(_5, _6),
                                    add(_7, _8)
                                )
                            ),
                            add(
                                add(_9, _10),
                                add(_11, _12)
                            )
                        )
                    )
                    return(0, 32)
                }
                // empty_body_arg_side_effect
                case 3 {
                    function f() -> ret {
                        mstore(0, 0xdeadbeef)
                        log0(0, 32)
                    }
                    function g(a) {}
                    g(f())
                }
                // leave_in_function
                case 4 {
                    function f() {
                        mstore(0, 0xdeadbeef01)
                        log0(0, 32)
                        leave
                        mstore(0, 0xdeadbeef02)
                        log0(0, 32)
                    }
                    f()
                }
                // return_in_function
                case 5 {
                    function f() {
                        mstore(0, 0xdeadbeef01)
                        log0(0, 32)
                        return(0, 32)
                        mstore(0, 0xdeadbeef02)
                        log0(0, 32)
                    }
                    f()
                }
                // revert_in_function
                case 6 {
                    function f() {
                        mstore(0, 0xdeadbeef01)
                        log0(0, 32)
                        revert(0, 32)
                        mstore(0, 0xdeadbeef02)
                        log0(0, 32)
                    }
                    f()
                }
                // default_ret_value
                case 7 {
                    function f() -> ret {}
                    mstore(0, f())
                    return(0, 32)
                }
                // recursive_function
                case 8 {
                    function sum(n) -> ret {
                        if eq(n, 0) {
                            ret := 0
                            leave
                        }
                        ret := add(
                            sum(sub(n, 1)),
                            n
                        )
                    }
                    mstore(0, sum(100)) // 1 + 2 + ... + 100 = 100 * (1 + 100) / 2 = 5050
                    return(0, 32)
                }
            }
        }
    }
}
