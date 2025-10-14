//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "e2179b8e",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! } ] }

// Report https://linear.app/matterlabs/issue/CPR-611/nested-external-calls-issue

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
                mstore(64, 128)
                if iszero(lt(calldatasize(), 4))
                {
                    switch shr(224, calldataload(0))
                    case 0x26121ff0 {
                        mstore(128, shl(224, 0xb8c9d365))
                        let _3 := call(gas(), address(), 0, 128, 4, 128, 32)
                        let memPos := mload(64)
                        return(memPos, 32)
                    }
                    case 0xb8c9d365 { external_fun_h() }
                    case 0xe2179b8e { external_fun_g() }
                }
                revert(0, 0)
            }

            function external_fun_g()
            {
                let _1 := mload(64)
                mstore(_1, shl(228, 0x026121ff))
                let _2 := call(gas(), address(), 0, _1, 4, _1, 32)
                return(_1, 32)
            }

            function external_fun_h()
            {
                let memPos := mload(64)
                mstore(memPos, 0x01)
                return(memPos, 32)
            }
        }
        data ".metadata" hex"a26469706673582212208f3883d9b5d20a0dc4de05a44ef431d061091099719869cc4d219181a2842c6c64736f6c634300080d0033"
    }
}
