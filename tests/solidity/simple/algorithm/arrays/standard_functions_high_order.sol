//! { "modes": [ "Y", "E", "I" ], "cases": [ {
//!     "name": "reverseTest",
//!     "inputs": [
//!         {
//!             "method": "reverseTest",
//!             "calldata": [
//!                 "7", "2", "1", "8", "10", "3", "5", "4", "9", "6"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "6", "9", "4", "5", "3", "10", "8", "1", "2", "7", "10"
//!     ]
//! }, {
//!     "name": "uniqueTest",
//!     "inputs": [
//!         {
//!             "method": "uniqueTest",
//!             "calldata": [
//!                 "7", "2", "7", "8", "10", "10", "5", "8", "9", "7"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "7", "2", "8", "10", "5", "9", "0", "0", "0", "0", "6"
//!     ]
//! }, {
//!     "name": "filterGreater",
//!     "inputs": [
//!         {
//!             "method": "filterGreater",
//!             "calldata": [
//!                 "7", "2", "7", "8", "10", "10", "5", "8", "9", "7"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "7", "7", "8", "10", "10", "8", "9", "7", "0", "0", "8"
//!     ]
//! }, {
//!     "name": "filterLess",
//!     "inputs": [
//!         {
//!             "method": "filterLess",
//!             "calldata": [
//!                 "7", "2", "7", "8", "10", "10", "5", "8", "9", "7"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "7", "2", "7", "5", "7", "0", "0", "0", "0", "0", "5"
//!     ]
//! }, {
//!     "name": "filterDivide",
//!     "inputs": [
//!         {
//!             "method": "filterDivide",
//!             "calldata": [
//!                 "7", "2", "7", "8", "10", "10", "5", "8", "9", "7"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "2", "8", "10", "10", "8", "0", "0", "0", "0", "0", "5"
//!     ]
//! }, {
//!     "name": "mapAdd",
//!     "inputs": [
//!         {
//!             "method": "mapAdd",
//!             "calldata": [
//!                 "7", "2", "1", "8", "10", "3", "5", "4", "9", "6", "28"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "35", "30", "29", "36", "38", "31", "33", "32", "37", "34", "10"
//!     ]
//! }, {
//!     "name": "mapSub",
//!     "inputs": [
//!         {
//!             "method": "mapSub",
//!             "calldata": [
//!                 "18", "12", "7", "9", "21", "6", "22", "14", "9", "34", "5"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "13", "7", "2", "4", "16", "1", "17", "9", "4", "29", "10"
//!     ]
//! }, {
//!     "name": "mapMul",
//!     "inputs": [
//!         {
//!             "method": "mapMul",
//!             "calldata": [
//!                 "7", "2", "1", "8", "10", "3", "5", "4", "9", "6", "7"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "49", "14", "7", "56", "70", "21", "35", "28", "63", "42", "10"
//!     ]
//! }, {
//!     "name": "mapDiv",
//!     "inputs": [
//!         {
//!             "method": "mapDiv",
//!             "calldata": [
//!                 "18", "12", "7", "9", "21", "6", "22", "14", "9", "34", "4"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "4", "3", "1", "2", "5", "1", "5", "3", "2", "8", "10"
//!     ]
//! }, {
//!     "name": "complex",
//!     "inputs": [
//!         {
//!             "method": "complex",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "4", "6", "2", "5", "0", "0", "0", "0", "0", "0", "4"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.19;
pragma experimental ABIEncoderV2;

contract Test {
    uint8 constant ARRAY_SIZE = 10;

    struct Vector {
        uint8[ARRAY_SIZE] array;
        uint8 size;
    }

    function reverseTest(uint8[ARRAY_SIZE] memory input) public pure returns(Vector memory) {
        return reverse(fromArray(input, ARRAY_SIZE));
    }

    function uniqueTest(uint8[ARRAY_SIZE] memory input) public pure returns(Vector memory) {
        return unique(fromArray(input, ARRAY_SIZE));
    }

    function filterGreater(uint8[ARRAY_SIZE] memory input) public pure returns(Vector memory) {
        return filter(fromArray(input, ARRAY_SIZE), greater5);
    }

    function filterLess(uint8[ARRAY_SIZE] memory input) public pure returns(Vector memory) {
        return filter(fromArray(input, ARRAY_SIZE), less8);
    }

    function filterDivide(uint8[ARRAY_SIZE] memory input) public pure returns(Vector memory) {
        return filter(fromArray(input, ARRAY_SIZE), divide2);
    }

    function mapAdd(uint8[ARRAY_SIZE] memory input) public pure returns(Vector memory) {
        return map(fromArray(input, ARRAY_SIZE), add28);
    }

    function mapSub(uint8[ARRAY_SIZE] memory input) public pure returns(Vector memory) {
        return map(fromArray(input, ARRAY_SIZE), sub5);
    }

    function mapMul(uint8[ARRAY_SIZE] memory input) public pure returns(Vector memory) {
        return map(fromArray(input, ARRAY_SIZE), mul7);
    }

    function mapDiv(uint8[ARRAY_SIZE] memory input) public pure returns(Vector memory) {
        return map(fromArray(input, ARRAY_SIZE), div4);
    }

    function complex() public pure returns(Vector memory) {
        uint8[ARRAY_SIZE] memory input = [18, 12, 7, 9, 21, 6, 22, 14, 9, 34];
        Vector memory output = fromArray(input, ARRAY_SIZE);
        output = filter(output, divide2);
        output = reverse(output);
        output = map(output, mul2);
        output = filter(output, less60);
        output = map(output, add4);
        output = filter(output, divide8);
        output = map(output, div7);

        return output;
    }

    function greater5(uint8 a) private pure returns(bool) {
        return a > 5;
    }

    function less8(uint8 a) private pure returns(bool) {
        return a < 8;
    }

    function divide2(uint8 a) private pure returns(bool) {
        return a % 2 == 0;
    }

    function add28(uint8 a) private pure returns(uint8) {
        return a + 28;
    }

    function sub5(uint8 a) private pure returns(uint8) {
        return a - 5;
    }

    function mul7(uint8 a) private pure returns(uint8) {
        return a * 7;
    }

    function div4(uint8 a) private pure returns(uint8) {
        return a / 4;
    }

    function mul2(uint8 a) private pure returns(uint8) {
        return a * 2;
    }

    function less60(uint8 a) private pure returns(bool) {
        return a < 60;
    }

    function add4(uint8 a) private pure returns(uint8) {
        return a + 4;
    }

    function divide8(uint8 a) private pure returns(bool) {
        return a % 8 == 0;
    }

    function div7(uint8 a) private pure returns(uint8) {
        return a / 7;
    }

    function fromArray(uint8[ARRAY_SIZE] memory array, uint8 size) private pure returns(Vector memory) {
        return Vector(array, size);
    }

    function copy(Vector memory vector) private pure returns(Vector memory) {
        Vector memory _new;
        _new.size = vector.size;
        for(uint8 i = 0; i < vector.size; i++) {
            _new.array[i] = vector.array[i];
        }
        return _new;
    }

    function reverse(Vector memory vector) private pure returns(Vector memory) {
        Vector memory result = copy(vector);
        for(uint i = 0; i < result.size / 2; i++) {
            uint8 tmp = result.array[i];
            result.array[i] = result.array[result.size - 1 - i];
            result.array[result.size - 1 - i] = tmp;
        }
        return result;
    }

    function unique(Vector memory vector) private pure returns(Vector memory) {
        Vector memory result;
        for(uint8 i = 0; i < vector.size; i++) {
            bool fl = true;
            for(uint8 j = 0; j < i; j++) {
                if (vector.array[i] == vector.array[j]) {
                    fl = false;
                    break;
                }
            }
            if (fl) {
                result.array[result.size] = vector.array[i];
                result.size += 1;
            }
        }
        return result;
    }

    function filter(Vector memory vector, function (uint8) pure returns(bool) f) private pure returns(Vector memory) {
        Vector memory result;
        for(uint8 i = 0; i < vector.size; i++) {
            if (f(vector.array[i])) {
                result.array[result.size] = vector.array[i];
                result.size += 1;
            }
        }
        return result;
    }

    function map(Vector memory vector, function (uint8) pure returns(uint8) f) private pure returns(Vector memory) {
        Vector memory result;
        result.size = vector.size;
        for(uint8 i = 0; i < vector.size; i++) {
            result.array[i] = f(vector.array[i]);
        }
        return result;
    }
}
