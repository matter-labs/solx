// SPDX-License-Identifier: Unlicensed

pragma solidity >=0.4.12;

contract Optimized {
    uint8 constant ARRAY_SIZE = 40;
    uint128 constant P = 257;
    uint128 constant MODULO = 1000000007;

    function complex() public pure returns(uint64) {
        uint8[ARRAY_SIZE] memory array;
        // generate array where first half equals second
        for(uint8 i = 0; i < ARRAY_SIZE; i++) {
            array[i] = (i % (ARRAY_SIZE / 2)) * (255 / (ARRAY_SIZE / 2 - 1));
        }

        bool result = true;
        for(uint8 j = 0; j < ARRAY_SIZE / 2; j++) {
            result = result && hash(array, 0, j + 1) == hash(array, ARRAY_SIZE / 2, ARRAY_SIZE / 2 + j + 1)
                && hash(array, j, ARRAY_SIZE / 2) == hash(array, j + ARRAY_SIZE / 2, ARRAY_SIZE);
        }
        if (result) {
            return 1;
        } else {
            return 0;
        }
    }

    function hash(uint8[ARRAY_SIZE] memory array, uint8 begin, uint8 end) private pure returns(uint128) {
        uint128 h = 0;
        for(uint8 i = begin; i < end; i++) {
            h = (h * P + array[i]) % MODULO;
        }
        return h;
    }
}