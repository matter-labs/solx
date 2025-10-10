//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "deadbeef",
//!             "calldata": [
//!                 "127", "-3"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "-42"
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
                mstore(256, sdiv(calldataload(4), calldataload(36)))
                return(256, 32)
            }
        }
    }
}
