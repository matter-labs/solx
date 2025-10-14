// SPDX-License-Identifier: MIT

pragma solidity >=0.6.9;

interface RequireInterface {
    function require_short() external view;
    
    function wrong_number_of_params(uint32 one, uint32 two, bytes memory three) external view returns(uint);
}
