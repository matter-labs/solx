// SPDX-License-Identifier: MIT OR Apache-2.0

pragma solidity ^0.8.0;

import "./TestContract.sol";

contract Main {
    function test() external {
        TestContract contract1 = new TestContract(1);
        TestContract contract2 = new TestContract(2);

        require(contract1.x() == 1);
        require(contract2.x() == 2);
        require(contract1.delegate(address(contract2), abi.encodeWithSelector(contract2.x.selector)) == 2);
    }
}
