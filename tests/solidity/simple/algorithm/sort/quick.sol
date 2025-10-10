//! { "modes": [ "Y", "E" ], "cases": [ {
//!     "name": "ascending",
//!     "inputs": [
//!         {
//!             "method": "quickSort",
//!             "calldata": [
//!                 "7", "2", "1", "8", "10", "3", "5", "4", "9", "6", "0", "10", "0"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"
//!     ]
//! }, {
//!     "name": "descending",
//!     "inputs": [
//!         {
//!             "method": "quickSort",
//!             "calldata": [
//!                 "7", "2", "1", "8", "10", "3", "5", "4", "9", "6", "0", "10", "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "10", "9", "8", "7", "6", "5", "4", "3", "2", "1"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.19;

contract Test {
    uint8 constant ARRAY_SIZE = 10;

    enum Direction {
        Ascending,
        Descending
    }

    function quickSort(
        uint8[ARRAY_SIZE] memory array,
        uint8 begin,
        uint8 end,
        Direction direction
    )
        public
        pure
        returns(uint8[ARRAY_SIZE] memory)
    {
        if (end - begin < 2) {
            return array;
        }

        uint8 pivot = array[end - 1];
        uint8 i = begin;

        for(uint8 j = begin; j < end - 1; j++) {
            if (
                (direction == Direction.Ascending && array[j] < pivot) ||
                (direction == Direction.Descending && array[j] > pivot)
            ) {
                uint8 temp = array[i];
                array[i] = array[j];
                array[j] = temp;
                i++;
            }
        }
        array[end - 1] = array[i];
        array[i] = pivot;

        quickSort(array, begin, i, direction);
        quickSort(array, i + 1, end, direction);
        return array;
    }
}