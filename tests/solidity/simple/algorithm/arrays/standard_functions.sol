//! { "cases": [ {
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
//!             "method": "filterTest",
//!             "calldata": [
//!                 "7", "2", "7", "8", "10", "10", "5", "8", "9", "7", "0", "5"
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
//!             "method": "filterTest",
//!             "calldata": [
//!                 "7", "2", "7", "8", "10", "10", "5", "8", "9", "7", "1", "8"
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
//!             "method": "filterTest",
//!             "calldata": [
//!                 "7", "2", "7", "8", "10", "10", "5", "8", "9", "7", "2", "2"
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
//!             "method": "mapTest",
//!             "calldata": [
//!                 "7", "2", "1", "8", "10", "3", "5", "4", "9", "6", "0", "28"
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
//!             "method": "mapTest",
//!             "calldata": [
//!                 "18", "12", "7", "9", "21", "6", "22", "14", "9", "34", "1", "5"
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
//!             "method": "mapTest",
//!             "calldata": [
//!                 "7", "2", "1", "8", "10", "3", "5", "4", "9", "6", "2", "7"
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
//!             "method": "mapTest",
//!             "calldata": [
//!                 "18", "12", "7", "9", "21", "6", "22", "14", "9", "34", "3", "4"
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

    enum Filter {
        Greater,
        Less,
        Divide
    }

    enum Mapping {
        Add,
        Sub,
        Mul,
        Div
    }

    function reverseTest(uint8[ARRAY_SIZE] memory input) public pure returns(Vector memory) {
        return reverse(fromArray(input, ARRAY_SIZE));
    }

    function uniqueTest(uint8[ARRAY_SIZE] memory input) public pure returns(Vector memory) {
        return unique(fromArray(input, ARRAY_SIZE));
    }

    function filterTest(uint8[ARRAY_SIZE] memory input, Filter _filter, uint8 param) public pure returns(Vector memory) {
        return filter(fromArray(input, ARRAY_SIZE), _filter, param);
    }

    function mapTest(uint8[ARRAY_SIZE] memory input, Mapping _mapping, uint8 param) public pure returns(Vector memory) {
        return map(fromArray(input, ARRAY_SIZE), _mapping, param);
    }

    function complex() public pure returns(Vector memory) {
        uint8[ARRAY_SIZE] memory input = [18, 12, 7, 9, 21, 6, 22, 14, 9, 34];
        Vector memory output = fromArray(input, ARRAY_SIZE);
        output = filter(output, Filter.Divide, 2); // 18 12 6 22 14 34
        output = reverse(output); // 34 14 22 6 12 18
        output = map(output, Mapping.Mul, 2); // 68 28 44 12 24 36
        output = filter(output, Filter.Less, 60); // 28 44 12 24 36
        output = map(output, Mapping.Add, 4); // 32 48 16 28 40
        output = filter(output, Filter.Divide, 8); // 32 48 16 40
        output = map(output, Mapping.Div, 7); // 4 6 2 5
        return output;
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

    function filter(Vector memory vector, Filter _filter, uint8 param) private pure returns(Vector memory) {
        Vector memory result;
        for(uint8 i = 0; i < vector.size; i++) {
            if (
                (_filter == Filter.Greater && vector.array[i] > param) ||
                (_filter == Filter.Less && vector.array[i] < param) ||
                (_filter == Filter.Divide && vector.array[i] % param == 0)
            ) {
                result.array[result.size] = vector.array[i];
                result.size += 1;
            }
        }
        return result;
    }

    function map(Vector memory vector, Mapping _mapping, uint8 param) private pure returns(Vector memory) {
        Vector memory result;
        result.size = vector.size;
        for(uint8 i = 0; i < vector.size; i++) {
            if (_mapping == Mapping.Add) {
                result.array[i] = vector.array[i] + param;
            } else if (_mapping == Mapping.Sub) {
                result.array[i] = vector.array[i] - param;
            } else if (_mapping == Mapping.Mul) {
                result.array[i] = vector.array[i] * param;
            } else if (_mapping == Mapping.Div) {
                result.array[i] = vector.array[i] / param;
            }
        }
        return result;
    }
}
