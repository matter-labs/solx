//! { "cases": [ {
//!     "name": "main",
//!     "inputs": [
//!         {
//!             "method": "main",
//!             "calldata": [
//!                 "42"
//!             ],
//!             "storage": { "Test.address": [
//!                 "20", "15", "10", "5"
//!             ] }
//!         }
//!     ],
//!     "expected": [
//!         "50"
//!     ]
//! } ] }

contract Test {
    uint8 constant TEST = 42;

    struct Data {
        uint256 a;
        uint256 b;
        uint256 c;
        uint256 d;
    }

    Data data;

    function main(uint8 argument) public returns(uint8) {
        uint8 sum = 0;
        sum += uint8(data.a);
        sum += uint8(data.b);
        sum += uint8(data.c);
        sum += uint8(data.d);
        return sum;
    }
}
