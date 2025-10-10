//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [
//!                 "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
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
                let expr
                switch calldataload(0) /* 1 */
                case 0 {
                    expr := 0
                }
                default
                {
                    expr := 0x01
                }
                if gt(calldataload(0) /* 1 */, not(expr))
                {
                    revert(0, 0)
                }
                mstore(0, expr)
                return(0, 32)
            }
        }
    }
}