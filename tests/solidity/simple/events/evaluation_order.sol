//! { "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!             "42",
//!             "25",
//!             "5"
//!         ],
//!         "events": [
//!             {
//!                 "topics": [
//!                     "5",
//!                     "25",
//!                     "42"
//!                 ],
//!                 "values": []
//!             }
//!         ]
//!     }
//! } ] }

contract Test {
    function main() external {
        assembly {
            function func(input) -> output {
                let currentPtr := mload(0x40)
                mstore(currentPtr, input)
                mstore(0x40, add(currentPtr, 0x20))
                output := input
            }

            log3(0x0, 0x0, func(5), func(25), func(42))

            let beginPtr := sub(mload(0x40), 0x60)
            return(beginPtr, 0x60)
        }
    }
}
