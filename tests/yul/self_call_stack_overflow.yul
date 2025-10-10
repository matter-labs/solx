//! { "cases": [ {
//!     "name": "10_failure",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [
//!                 "0", "1"
//!             ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [ "0" ],
//!         "exception": true
//!     }
//! } ] }

// Report https://linear.app/matterlabs/issue/CPR-657/panic-is-lost-on-infinite-recursion

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
                let result := staticcall(0, address(), 0, 0, 0, 0)
                mstore(0, result)
                if result {
                    return(0, 32)
                }
                revert(0, 32)
            }
        }
    }
}
