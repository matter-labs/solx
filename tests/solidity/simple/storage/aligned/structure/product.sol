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
//!         "15000"
//!     ]
//! } ] }

contract Test {
    struct Data {
        uint256 a;
        uint256 b;
        uint256 c;
        uint256 d;
    }

    Data data;

    function main(uint16 argument) public returns(uint16) {
        uint16 product = 1;
        product *= uint16(data.a);
        product *= uint16(data.b);
        product *= uint16(data.c);
        product *= uint16(data.d);
        return product;
    }
}
