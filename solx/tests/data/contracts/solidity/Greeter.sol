// SPDX-License-Identifier: Unlicensed

pragma solidity >=0.4.12;

library GreeterHelper {
    function addPrefix(Greeter greeter, string memory great) public view returns (string memory) {
        return string.concat(greeter.prefix(),great);
    }
}

contract Greeter {
    string private greeting;
    string private _prefix;

    constructor(string memory _greeting) {
        greeting = _greeting;
        _prefix = "The greating is:";
    }

    function prefix() public view returns (string memory) {
        return _prefix;
    }

    function greet() public view returns (string memory) {
        return GreeterHelper.addPrefix(this, greeting);
    }

    function setGreeting(string memory _greeting) public {
        greeting = _greeting;
    }
}
