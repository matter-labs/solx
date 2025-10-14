//! { "modes": [
//!     "Y >=0.8.1",
//!     "E", "I"
//! ], "cases": [ {
//!     "name": "first",
//!     "inputs": [
//!         {
//!             "method": "first",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "second",
//!     "inputs": [
//!         {
//!             "method": "second",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "third",
//!     "inputs": [
//!         {
//!             "method": "third",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "four",
//!     "inputs": [
//!         {
//!             "method": "four",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "five",
//!     "inputs": [
//!         {
//!             "method": "five",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "six",
//!     "inputs": [
//!         {
//!             "method": "six",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "seven",
//!     "inputs": [
//!         {
//!             "method": "seven",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "eight",
//!     "inputs": [
//!         {
//!             "method": "eight",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "nine",
//!     "inputs": [
//!         {
//!             "method": "nine",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "ten",
//!     "inputs": [
//!         {
//!             "method": "ten",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "eleven",
//!     "inputs": [
//!         {
//!             "method": "eleven",
//!             "calldata": [
//!                 "0x20", "0x03", "1", "2", "3"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "twelve",
//!     "inputs": [
//!         {
//!             "method": "twelve",
//!             "calldata": [
//!                 "0x20", "0x03", "1", "2", "3"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.12;

contract Test {
    uint256[2][] arr;

    function first() public returns (uint256) {
        uint256[] memory x = new uint256[](42);
        delete x;
        assert(x.length == 0);
        return 0;
    }

    function second() public returns (uint256) {
        uint256[][] memory x = new uint256[][](42);
        delete x;
        assert(x.length == 0);
        return 0;
    }

    function third() public returns (uint256) {
        uint256[][] memory x = new uint256[][](42);
        delete x;
        delete x;
        assert(x.length == 0);
        return 0;
    }

    function four() public returns (uint256) {
        uint256[][] memory x = new uint256[][](42);
        x[1] = new uint256[](7);
        delete x[0];
        assert(x[1].length == 7);
        return 0;
    }

    function five() public returns (uint256) {
        uint256[5] memory x;
        delete x;
        assert(x.length == 5);
        return 0;
    }

    function six() public returns (uint256) {
        uint256[5] memory x;
        x[0] = 99;
        delete x;
        assert(x[0] == 0);
        return 0;
    }

    function seven() public returns (uint256) {
        arr.push([11, 22]);
        delete arr[0];
        assert(arr[0].length == 2);
        return 0;
    }

    function eight() public returns (uint256) {
        uint256[2][] memory x = new uint256[2][](3);
        x[0] = [uint(99), 11];
        delete x;
        assert(x.length == 0);
        return 0;
    }

    function nine() public returns (uint256) {
        uint256[2][] memory x = new uint256[2][](3);
        x[0] = [uint(99), 11];
        delete x[0];
        assert(x[2].length == 2);
        return 0;
    }

    function ten() public returns (uint256) {
        uint256[5][] memory x = new uint256[5][](0);
        delete x;
        assert(x.length == 0);
        return 0;
    }

    function eleven(uint256[] memory array) public returns (uint256) {
        delete array;
        assert(array.length == 0);
        return 0;
    }

    function twelve(uint256[3] memory array) public returns (uint256) {
        delete array;
        assert(array.length == 3);
        return 0;
    }
}
