//! { "cases": [ {
//!     "name": "require_short",
//!     "inputs": [
//!         {
//!             "method": "require_short",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!             "0x08C379A000000000000000000000000000000000000000000000000000000000",
//!             "0x0000002000000000000000000000000000000000000000000000000000000000",
//!             "0x0000000573686F72740000000000000000000000000000000000000000000000",
//!             "0x0000000000000000000000000000000000000000000000000000000000000000"
//!         ],
//!         "exception": true
//!     }
//! }, {
//!     "name": "require_long",
//!     "inputs": [
//!         {
//!             "method": "require_long",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!             "0x08C379A000000000000000000000000000000000000000000000000000000000",
//!             "0x0000002000000000000000000000000000000000000000000000000000000000",
//!             "0x000000DD6C6C6F6E676C6F6E676C6F6E6774657874626C6F6E676C6F6E676C6F",
//!             "0x6E6774657874626C6F6E676C6F6E676C6F6E6774657874626C6F6E676C6F6E67",
//!             "0x6C6F6E6774657874626C6F6E676C6F6E676C6F6E6774657874626C6F6E676C6F",
//!             "0x6E676C6F6E6774657874626C6F6E676C6F6E676C6F6E6774657874626C6F6E67",
//!             "0x6C6F6E676C6F6E6774657874626C6F6E676C6F6E676C6F6E6774657874626C6F",
//!             "0x6E676C6F6E676C6F6E6774657874626C6F6E676C6F6E676C6F6E677465787462",
//!             "0x6C6F6E676C6F6E676C6F6E6774657874626F6E676C6F6E676C6F6E6774657874",
//!             "0x6200000000000000000000000000000000000000000000000000000000000000"
//!         ],
//!         "exception": true
//!     }
//! }, {
//!     "name": "new_error",
//!     "inputs": [
//!         {
//!             "method": "new_error",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!             "0x157BEA6000000000000000000000000000000000000000000000000000000000",
//!             "0x0000000100000000000000000000000000000000000000000000000000000000",
//!             "0x0000000200000000000000000000000000000000000000000000000000000000",
//!             "0x0000000100000000000000000000000000000000000000000000000000000000",
//!             "0x0000008000000000000000000000000000000000000000000000000000000000",
//!             "0x0000000C64617461646174616461746100000000000000000000000000000000",
//!             "0x0000000000000000000000000000000000000000000000000000000000000000"
//!         ],
//!         "exception": true
//!     }
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.4;

contract Test {
    error TestError(uint256 one, uint256 two, uint256 three, string data);

    function require_short() public pure {
        require(false, "short");
    }

    function require_long() public pure {
        require(false, "llonglonglongtextblonglonglongtextblonglonglongtextblonglonglongtextblonglonglongtextblonglonglongtextblonglonglongtextblonglonglongtextblonglonglongtextblonglonglongtextblonglonglongtextblonglonglongtextbonglonglongtextb");
    }

    function new_error() public pure {
        revert TestError({one: 1, two: 2, three: 1, data: "datadatadata"});
    }
}
