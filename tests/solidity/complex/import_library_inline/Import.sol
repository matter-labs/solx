// SPDX-License-Identifier: MIT

pragma solidity >=0.6.2;

// import Foo.sol from current directory
import "./Foo.sol";

contract Import {
    // Test Foo.sol by getting it's name.
    function getFooName() public pure returns (string memory) {
        return Foo.name;
    }
}
