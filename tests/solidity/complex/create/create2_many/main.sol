// SPDX-License-Identifier: MIT

pragma solidity >=0.6.2;

import "./callable.sol";

contract Main {
    function main(uint8 n) external returns(uint256) {
        address[254] memory addresses;
        for (uint8 i = 0; i < n; i++) {
            addresses[i] = address(new Callable{salt: bytes32(uint256(i))}());
            Callable(addresses[i]).set([uint256(i) + 1, uint256(i) + 3], i + 2);
        }

        uint256 result = 0;
        for (uint8 i = 0; i < n; i++) {
            uint256[2] memory a;
            uint8 b;
            (a, b) = Callable(addresses[i]).get();
            result += a[0] * a[1] * b;
        }

        return result;
    }
}
