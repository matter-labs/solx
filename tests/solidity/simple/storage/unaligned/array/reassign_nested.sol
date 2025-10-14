//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "42"
//!             ],
//!             "storage": { "Test.address": [
//!                 "0x04030201",
//!                 "0x08070605",
//!                 "0x0c0b0a09",
//!                 "0x100f0e0d"
//!             ] }
//!         }
//!     ],
//!     "expected": [
//!         "142"
//!     ]
//! } ] }

contract Test {
    uint8[4][4] data;

    uint8 constant TEST = 42;

    function main(uint8 argument) public returns(uint8) {
        data[3][3] += argument;

        return argument + TEST + data[3][3];
    }
}
