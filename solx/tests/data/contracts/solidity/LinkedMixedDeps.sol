// SPDX-License-Identifier: Unlicensed

pragma solidity >=0.4.12;

contract InnerContract {
    uint256 x;
    uint256 y;

    constructor(uint _x, uint _y) {
        x = _x;
        y = _y;
    }

    function getSum() public view returns (uint256 result) {
        result = x + y;
    }
}

library UpperLibrary {
    function add(uint256 a, uint256 b) external returns (uint256 result) {
        result = new InnerContract(a, b).getSum();
    }
}

contract UpperContract {
    function performAlgorithm(uint256 a, uint256 b) public returns (uint256) {
        uint sum = 0;
        if (a > b) {
            while (true) {
                sum += UpperLibrary.add(a, b);
            }
        }
        return sum;
    }
}