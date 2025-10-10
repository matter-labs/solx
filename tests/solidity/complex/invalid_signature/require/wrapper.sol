// SPDX-License-Identifier: MIT

pragma solidity >=0.6.9;

import "./requireInterface.sol";

contract Wrapper {
    address private contract_req;

    function setContract (address _contract_re) public {
        contract_req = _contract_re;
    }

    function require_short() public view {
        RequireInterface(contract_req).require_short();
    }

    function wrong_number_of_params() public view returns(uint){
        return RequireInterface(contract_req).wrong_number_of_params(1, 2, new bytes(0));
    }

}
