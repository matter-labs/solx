//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "deadbeef",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! } ] }

// Report https://linear.app/matterlabs/issue/CPR-215/solidity-dynamic-array-length-bug

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
            mstore(64, 128)

            let expr_20_mpos := allocate_array(42)

            let expr_22_mpos := mload(array_index_access(expr_20_mpos, 0x00))

            let expr_23 := mload(expr_22_mpos)

            let memPos := allocate_memory(0)
            let memEnd := add(memPos, 32)
            mstore(memPos, 0)
            return(memPos, sub(memEnd, memPos))
            
            function allocate_array(length) -> memPtr
            {
                let allocSize := add(mul(length, 0x20), 0x20)
                memPtr := allocate_memory(allocSize)
                mstore(memPtr, length)

                let dataSize := add(mul(length, 0x20), 0x20)
                let dataStart := add(memPtr, 32)
                dataSize := sub(dataSize, 32)
                for { let i := 0 } lt(i, dataSize) { i := add(i, 32) }
                {
                    mstore(add(dataStart, i), zero_value_for_t_array$_t_uint256_$dyn_memory_ptr())
                }
            }

            function array_index_access(baseRef, index) -> addr
            {
                let offset := mul(index, 32)
                offset := add(offset, 32)
                addr := add(baseRef, offset)
            }

            function allocate_memory(size) -> memPtr
            {
                memPtr := mload(64)
                let newFreePtr := add(memPtr, and(add(size, 31), not(31)))
                mstore(64, newFreePtr)
            }

            function zero_value_for_t_array$_t_uint256_$dyn_memory_ptr() -> ret
            { ret := 96 }
        }
    }
}
