//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [
//!                 "0x111111111"
//!             ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!         ],
//!         "events": [
//!             {
//!                 "topics": [],
//!                 "values": []
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
            pop(abi_decode_tuple(0, 0))
            test(0)
            return(0, 0)

            function abi_decode_uint256() -> value
            {
                value := calldataload(0)
            }
            function abi_decode_tuple(a, b) -> value0
            {
                let offset := 0
                value0 := abi_decode_uint256()
            }
            function test(var_number_5)
            {
                let _1 := 0
                let _2 := 0
                log0(0, 0)
            }
        }
    }
}
