//! { "cases": [ {
//!     "name": "simple_statement",
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
//!     "name": "nested_statements",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "1" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [],
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
//!     "name": "return_in_middle",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "2" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!             "0xdeadbeef"
//!         ],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "revert_in_middle",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "3" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!             "0xdeadbeef"
//!         ],
//!         "exception": true
//!     }
//! }, {
//!     "name": "return_in_nested_statement",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "4" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [ "0xdeadbeef02" ],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef01" ]
//!             }
//!         ]
//!     }
//! }, {
//!     "name": "revert_in_nested_statement",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "5" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [ "0xdeadbeef02" ],
//!         "exception": true
//!     }
//! }, {
//!     "name": "complex",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "6" ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [],
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
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef04" ]
//!             },
//!             {
//!                 "topics": [],
//!                 "values": [ "0xdeadbeef05" ]
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
                // simple_statement
                case 0 {
                    {
                        mstore(0, 0xdeadbeef)
                        log0(0, 32)
                    }
                }
                // nested_statements
                case 1 {
                    {
                        {
                            mstore(0, 0xdeadbeef01)
                            log0(0, 32)
                        }
                        mstore(0, 0xdeadbeef02)
                        log0(0, 32)
                    }
                }
                // return_in_middle
                case 2 {
                    {
                        mstore(0, 0xdeadbeef)
                        log0(0, 32)
                        return(0, 32)
                        mstore(0, 0xdeadbeef02)
                        log0(0, 32)
                    }
                }
                // revert_in_middle
                case 3 {
                    {
                        mstore(0, 0xdeadbeef)
                        log0(0, 32)
                        revert(0, 32)
                        mstore(0, 0xdeadbeef02)
                        log0(0, 32)
                    }
                }
                // return_in_nested_statement
                case 4 {
                    {
                        mstore(0, 0xdeadbeef01)
                        log0(0, 32)
                        {
                            mstore(0, 0xdeadbeef02)
                            return(0, 32)
                        }
                    }
                }
                // revert_in_nested_statement
                case 5 {
                    {
                        mstore(0, 0xdeadbeef01)
                        log0(0, 32)
                        {
                            mstore(0, 0xdeadbeef02)
                            revert(0, 32)
                        }
                    }
                }
                // complex
                case 6 {
                    {{
                        mstore(0, 0xdeadbeef01)
                        log0(0, 32)
                        {{{{{{{{{{{{{{{
                            {{{{{{{{{{{
                                mstore(0, 0xdeadbeef02)
                                log0(0, 32)
                            }}}}}}}}}}}
                            mstore(0, 0xdeadbeef03)
                            log0(0, 32)
                        }}}}}}}}}}}}}}}
                        mstore(0, 0xdeadbeef04)
                        log0(0, 32)
                    }}
                    mstore(0, 0xdeadbeef05)
                    log0(0, 32)
                }
            }
        }
    }
}