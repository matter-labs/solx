//! { "cases": [ {
//!     "name": "returnMany",
//!     "inputs": [
//!         {
//!             "method": "returnMany",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1", "1", "2"
//!     ]
//! }, {
//!     "name": "named",
//!     "inputs": [
//!         {
//!             "method": "named",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1", "1", "2"
//!     ]
//! }, {
//!     "name": "assigned",
//!     "inputs": [
//!         {
//!             "method": "assigned",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1", "1", "2"
//!     ]
//! }, {
//!     "name": "destructingAssigments",
//!     "inputs": [
//!         {
//!             "method": "destructingAssigments",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1", "1", "2", "4", "6"
//!     ]
//! }, {
//!     "name": "arrayInput",
//!     "inputs": [
//!         {
//!             "method": "arrayInput",
//!             "calldata": [
//!                 "0x20",
//!                 "0"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!     ]
//! }, {
//!     "name": "arrayOutput",
//!     "inputs": [
//!         {
//!             "method": "arrayOutput",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0x20",
//!         "0"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT
pragma solidity >=0.4.24;

contract Test {
    // Functions can return multiple values.
    function returnMany()
        public
        pure
        returns (
            uint,
            bool,
            uint
        )
    {
        return (1, true, 2);
    }

    // Return values can be named.
    function named()
        public
        pure
        returns (
            uint x,
            bool b,
            uint y
        )
    {
        return (1, true, 2);
    }

    // Return values can be assigned to their name.
    // In this case the return statement can be omitted.
    function assigned()
        public
        pure
        returns (
            uint x,
            bool b,
            uint y
        )
    {
        x = 1;
        b = true;
        y = 2;
    }

    // Use destructing assignment when calling another
    // function that returns multiple values.
    function destructingAssigments()
        public
        pure
        returns (
            uint,
            bool,
            uint,
            uint,
            uint
        )
    {
        (uint i, bool b, uint j) = returnMany();

        // Values can be left out.
        (uint x, , uint y) = (4, 5, 6);

        return (i, b, j, x, y);
    }

    // Cannot use map for neither input nor output

    // Can use array for input
    function arrayInput(uint[] memory _arr) public {}

    // Can use array for output
    uint[] public arr;

    function arrayOutput() public view returns (uint[] memory) {
        return arr;
    }
}
