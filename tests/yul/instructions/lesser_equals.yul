//! { "cases": [ {
//!     "name": "5_10",
//!     "inputs": [
//!         {
//!             "method": "deadbeef",
//!             "calldata": [
//!                 "5", "10"
//!             ]
//!         }
//!     ],
//!     "expected": [ "1" ]
//! },
//! {
//!     "name": "10_10",
//!     "inputs": [
//!         {
//!             "method": "deadbeef",
//!             "calldata": [
//!                 "10", "10"
//!             ]
//!         }
//!     ],
//!     "expected": [ "1" ]
//! },
//! {
//!     "name": "15_10",
//!     "inputs": [
//!         {
//!             "method": "deadbeef",
//!             "calldata": [
//!                 "15", "10"
//!             ]
//!         }
//!     ],
//!     "expected": [ "0" ]
//! },
//! {
//!     "name": "-5_-10",
//!     "inputs": [
//!         {
//!             "method": "deadbeef",
//!             "calldata": [
//!                 "-5", "-10"
//!             ]
//!         }
//!     ],
//!     "expected": [ "0" ]
//! },
//! {
//!     "name": "-10_-10",
//!     "inputs": [
//!         {
//!             "method": "deadbeef",
//!             "calldata": [
//!                 "-10", "-10"
//!             ]
//!         }
//!     ],
//!     "expected": [ "1" ]
//! },
//! {
//!     "name": "-15_-10",
//!     "inputs": [
//!         {
//!             "method": "deadbeef",
//!             "calldata": [
//!                 "-15", "-10"
//!             ]
//!         }
//!     ],
//!     "expected": [ "1" ]
//! },
//! {
//!     "name": "-5_10",
//!     "inputs": [
//!         {
//!             "method": "deadbeef",
//!             "calldata": [
//!                 "-5", "10"
//!             ]
//!         }
//!     ],
//!     "expected": [ "1" ]
//! },
//! {
//!     "name": "5_-10",
//!     "inputs": [
//!         {
//!             "method": "deadbeef",
//!             "calldata": [
//!                 "5", "-10"
//!             ]
//!         }
//!     ],
//!     "expected": [ "0" ]
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
            mstore(0, or(slt(calldataload(4), calldataload(36)), eq(calldataload(4), calldataload(36))))
            return(0, 32)
        }
    }
}
