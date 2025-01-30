// SPDX-License-Identifier: Unlicensed

pragma solidity >=0.4.12;

// A simple library with at least one external method
library SimpleLibrary {
    function add(uint256 a, uint256 b) external pure returns (uint256) {
        return a + b;
    }
}

// A contract calling that library
contract SimpleContract {
    using SimpleLibrary for uint256;

    function performAlgorithm(uint256 a, uint256 b) public pure returns (uint256) {
        uint sum = 0;
        if (a > b) {
            while (true) {
                sum += a.add(b);
            }
        }
        return sum;
    }
}