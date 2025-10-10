//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "42"
//!             ],
//!             "storage": { "Test.address": [
//!                 "99", "100", "101"
//!             ] }
//!         }
//!     ],
//!     "expected": [
//!         "226"
//!     ]
//! } ] }

contract Test {
    uint8 constant TEST = 42;

    struct Data {
        uint256 value;
        uint256 next;
        uint256 last;
    }

    Data data;

    function main(uint8 argument) public returns(uint8) {
        data.next += uint256(argument);

        return argument + TEST + uint8(data.next);
    }
}
