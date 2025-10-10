//! { "cases": [ {
//!     "name": "first",
//!     "inputs": [
//!         {
//!             "method": "deadbeef",
//!             "calldata": [
//!                 "0x0102030405060708", "31"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0x08"
//!     ]
//! }, {
//!     "name": "middle",
//!     "inputs": [
//!         {
//!             "method": "deadbeef",
//!             "calldata": [
//!                 "0x0102030405060708", "28"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0x05"
//!     ]
//! }, {
//!     "name": "last",
//!     "inputs": [
//!         {
//!             "method": "deadbeef",
//!             "calldata": [
//!                 "0x0102030405060708", "24"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0x01"
//!     ]
//! }, {
//!     "name": "out_of_range",
//!     "inputs": [
//!         {
//!             "method": "deadbeef",
//!             "calldata": [
//!                 "0x0102030405060708", "0"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0x00"
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
                let value := calldataload(4)
                let index := calldataload(36)

                mstore(0, byte(index, value))
                return(0, 32)
            }
        }
    }
}

