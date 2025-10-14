// SPDX-License-Identifier: MIT

pragma solidity >=0.6.0;

contract OtherContract {
    function externalMultiplyByTwo(uint256 num) public pure returns (uint256) {
        return num * 2;
    }
}

contract Test {
    function (uint) internal returns (uint) funcPtr;
    OtherContract public otherContract;

    constructor(OtherContract _otherContract) public {
        funcPtr = recursiveMultiplyByTwo;
        otherContract = _otherContract;
    }

    function recursiveMultiplyByTwo(uint num) internal returns (uint) {
        if (num <= 1) return num;
        return 2 * funcPtr(num - 1);
    }

    function complexOperation(uint num) public returns (uint) {
        uint result = 0;
        for(uint i = 0; i < num; i++) {
            try this.externalCall(i) returns (uint externalResult) {
                result += externalResult;
            } catch {
                result += i;
            }
            result += funcPtr(i);
        }
        return result;
    }

    function externalCall(uint num) public returns (uint) {
        (bool success, bytes memory data) = address(otherContract).call(abi.encodeWithSignature("externalMultiplyByTwo(uint256)", num));
        require(success, "External call failed");
        return abi.decode(data, (uint));
    }
}
