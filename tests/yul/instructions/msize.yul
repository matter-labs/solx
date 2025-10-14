//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "deadbeef",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!         "128"
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
                mstore(0, 1)
                mstore(32, 1)
                mstore(64, 1)
                mstore(96, 1)
                mstore(128, msize())
                return(128, 32)
            }
        }
    }
}
