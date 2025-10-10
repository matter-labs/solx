//! { "cases": [ {
//!     "name": "simple_statement",
//!     "inputs": [
//!         {
//!             "method": "simple_statement",
//!             "calldata": []
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
//!             "method": "nested_statements",
//!             "calldata": []
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
//!             "method": "return_in_middle",
//!             "calldata": []
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
//!             "method": "revert_in_middle",
//!             "calldata": []
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
//!             "method": "return_in_nested_statement",
//!             "calldata": []
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
//!             "method": "revert_in_nested_statement",
//!             "calldata": []
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
//!             "method": "complex",
//!             "calldata": []
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

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.12;

contract Test {
    function simple_statement() external {
        assembly {
            {
                mstore(0, 0xdeadbeef)
                log0(0, 32)
            }
        }
    }


    function nested_statements() external {
        assembly {
            {
                {
                    mstore(0, 0xdeadbeef01)
                    log0(0, 32)
                }
                mstore(0, 0xdeadbeef02)
                log0(0, 32)
            }
        }
    }

    function return_in_middle() external {
        assembly {
            {
                mstore(0, 0xdeadbeef)
                log0(0, 32)
                return(0, 32)
                mstore(0, 0xdeadbeef02)
                log0(0, 32)
            }
        }
    }

    function revert_in_middle() external {
        assembly {
            {
                mstore(0, 0xdeadbeef)
                log0(0, 32)
                revert(0, 32)
                mstore(0, 0xdeadbeef02)
                log0(0, 32)
            }
        }
    }

    function return_in_nested_statement() external {
        assembly {
            {
                mstore(0, 0xdeadbeef01)
                log0(0, 32)
                {
                    mstore(0, 0xdeadbeef02)
                    return(0, 32)
                }
            }
        }
    }

    function revert_in_nested_statement() external {
        assembly {
            {
                mstore(0, 0xdeadbeef01)
                log0(0, 32)
                {
                    mstore(0, 0xdeadbeef02)
                    revert(0, 32)
                }
            }
        }
    }

    function complex() external {
        assembly {
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
