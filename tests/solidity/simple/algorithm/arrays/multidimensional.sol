//! { "cases": [ {
//!     "name": "array_2d",
//!     "inputs": [
//!         {
//!             "method": "array_2d",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! }, {
//!     "name": "array_3d",
//!     "inputs": [
//!         {
//!             "method": "array_3d",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! }, {
//!     "name": "array_4d",
//!     "inputs": [
//!         {
//!             "method": "array_4d",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.5.0;

contract Test {
    uint8 constant ARRAY_SIZE_2D = 8;
    uint8 constant ARRAY_SIZE_3D = 3;
    uint8 constant ARRAY_SIZE_4D = 1;

    function array_2d() public pure returns(uint64) {
        uint8[ARRAY_SIZE_2D][ARRAY_SIZE_2D] memory array;

        for(uint8 i = 0; i < ARRAY_SIZE_2D; i++) {
            for(uint8 j = 0; j < ARRAY_SIZE_2D; j++) {
                array[i][j] = i*ARRAY_SIZE_2D + j;
            }
        }

        bool result = true;
        // other order

        for(uint8 j = 0; j < ARRAY_SIZE_2D; j++) {
            for(uint8 i = 0; i < ARRAY_SIZE_2D; i++) {
                result = result && array[i][j] == i*ARRAY_SIZE_2D + j;
            }
        }

        if (result) {
            return 1;
        } else {
            return 0;
        }
    }

    function array_3d() public pure returns(uint64) {
        uint8[ARRAY_SIZE_3D][ARRAY_SIZE_3D][ARRAY_SIZE_3D] memory array;

        for(uint8 i = 0; i < ARRAY_SIZE_3D; i++) {
            for(uint8 j = 0; j < ARRAY_SIZE_3D; j++) {
                for(uint8 p = 0; p < ARRAY_SIZE_3D; p++) {
                    array[i][j][p] = i*ARRAY_SIZE_3D*ARRAY_SIZE_3D + j*ARRAY_SIZE_3D + p;
                }
            }
        }

        bool result = true;
        // other order

        for(uint8 p = 0; p < ARRAY_SIZE_3D; p++) {
            for(uint8 j = 0; j < ARRAY_SIZE_3D; j++) {
                for(uint8 i = 0; i < ARRAY_SIZE_3D; i++) {
                    result = result && array[i][j][p] == i*ARRAY_SIZE_3D*ARRAY_SIZE_3D + j*ARRAY_SIZE_3D + p;
                }
            }
        }

        if (result) {
            return 1;
        } else {
            return 0;
        }
    }

    function array_4d() public pure returns(uint64) {
        uint8[ARRAY_SIZE_4D][ARRAY_SIZE_4D][ARRAY_SIZE_4D][ARRAY_SIZE_4D] memory array;

        for(uint8 i = 0; i < ARRAY_SIZE_4D; i++) {
            for(uint8 j = 0; j < ARRAY_SIZE_4D; j++) {
                for(uint8 p = 0; p < ARRAY_SIZE_4D; p++) {
                    for(uint8 h = 0; h < ARRAY_SIZE_4D; h++) {
                        array[i][j][p][h] = i*ARRAY_SIZE_4D*ARRAY_SIZE_4D*ARRAY_SIZE_4D
                        + j*ARRAY_SIZE_4D*ARRAY_SIZE_4D + p*ARRAY_SIZE_4D + h;
                    }
                }
            }
        }

        bool result = true;
        // other order

        for(uint8 h = 0; h < ARRAY_SIZE_4D; h++) {
            for(uint8 p = 0; p < ARRAY_SIZE_4D; p++) {
                for(uint8 j = 0; j < ARRAY_SIZE_4D; j++) {
                    for(uint8 i = 0; i < ARRAY_SIZE_4D; i++) {
                        result = result && array[i][j][p][h] == i*ARRAY_SIZE_4D*ARRAY_SIZE_4D*ARRAY_SIZE_4D
                        + j*ARRAY_SIZE_4D*ARRAY_SIZE_4D + p*ARRAY_SIZE_4D + h;
                    }
                }
            }
        }

        if (result) {
            return 1;
        } else {
            return 0;
        }
    }
}