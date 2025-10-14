// SPDX-License-Identifier: MIT

pragma solidity >=0.8.4;

contract ERC20Minimal {
    mapping(address => uint256) private _balances;

    function balanceOf(address account) external view returns (uint256) {
        return _balances[account];
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        _balances[to] += amount;
        return true;
    }
}