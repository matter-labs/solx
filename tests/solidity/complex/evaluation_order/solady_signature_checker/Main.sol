// SPDX-License-Identifier: MIT

pragma solidity >=0.8.4;

import "./Wallet.sol";
import "./SignatureCheckerLib.sol";

contract Main {
   function testValidSignatureNow(Wallet _wallet) external {
        _wallet.setShouldReturnMagic(true);
        bool success = SignatureCheckerLib.isValidSignatureNow(address(_wallet), keccak256(""), "some_bytes");
        require(success);
   }

   function testInvalidSignatureNow(Wallet _wallet) external {
        _wallet.setShouldReturnMagic(false);
        bool success = SignatureCheckerLib.isValidSignatureNow(address(_wallet), keccak256(""), "some_bytes");
        require(!success);
    }

    function testValidERC1271SignatureNow(Wallet _wallet) external {
        _wallet.setShouldReturnMagic(true);
        bool success = SignatureCheckerLib.isValidERC1271SignatureNow(address(_wallet), keccak256(""), "some_bytes");
        require(success);
    }

    function testInvalidERC1271SignatureNow(Wallet _wallet) external {
        _wallet.setShouldReturnMagic(false);
        bool success = SignatureCheckerLib.isValidERC1271SignatureNow(address(_wallet), keccak256(""), "some_bytes");
        require(!success);
    }
}
