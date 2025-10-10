//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "deadbeef",
//!             "calldata": [
//!                 "0", "0"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! } ] }

//
// Report https://linear.app/matterlabs/issue/CPR-182/calldata-offset-adjustment-lowering
//

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
                let _1 := 0
                let value0 := abi_decode_uint8(4)
                let value1 := abi_decode_uint8(36)
                let x := and(value0, 0xff)
                let y := and(value1, 0xff)
                let memPos := allocate_memory(_1)
                return(memPos, sub(abi_encode_uint8(memPos, add(x, y)), memPos))
            }
            function abi_decode_uint8(offset) -> value
            {
                value := calldataload(offset)
                if iszero(eq(value, and(value, 0xff))) { revert(0, 0) }
            }
            function abi_encode_uint8(headStart, value0) -> tail
            {
                tail := add(headStart, 32)
                mstore(headStart, and(value0, 0xff))
            }
            function allocate_memory(size) -> memPtr
            {
                memPtr := mload(64)
                let newFreePtr := add(memPtr, and(add(size, 31), not(31)))
                mstore(64, newFreePtr)
            }
        }
    }
}
