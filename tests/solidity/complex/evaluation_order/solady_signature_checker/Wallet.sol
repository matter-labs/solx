// SPDX-License-Identifier: MIT

pragma solidity >=0.8.4;

contract Wallet {
    // bytes4(keccak256("isValidSignature(bytes32,bytes)")
    bytes4 constant internal MAGICVALUE = 0x1626ba7e;

    bool shouldReturnMagic;

    function setShouldReturnMagic(bool _shouldReturnMagic) external {
        shouldReturnMagic = _shouldReturnMagic;
    }

    function isValidSignature(bytes32, bytes memory) external view returns (bytes4) {
        if (shouldReturnMagic) {
            return MAGICVALUE;
        } else {
            revert();
        }
    }
}
