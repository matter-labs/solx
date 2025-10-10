//! { "cases": [ {
//!     "name": "ok",
//!     "inputs": [
//!         {
//!             "method": "deposit",
//!             "calldata": [
//!                 "20"
//!             ]
//!         },
//!         {
//!             "method": "withdraw",
//!             "calldata": [
//!                 "10"
//!             ]
//!         },
//!         {
//!             "method": "balance",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "10"
//!     ]
//! }, {
//!     "name": "MAX_UINT",
//!     "inputs": [
//!         {
//!             "method": "MAX_UINT",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
//!     ]
//! }, {
//!     "name": "overflow",
//!     "inputs": [
//!         {
//!             "method": "deposit",
//!             "calldata": [
//!                 "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
//!             ]
//!         },
//!         {
//!             "method": "deposit",
//!             "calldata": [
//!                 "1"
//!             ]
//!         }
//!     ],
//!     "expected": [ {
//!         "compiler_version": "<0.8",
//!         "return_data": [
//!             "0x08c379a000000000000000000000000000000000000000000000000000000000",
//!             "0x0000002000000000000000000000000000000000000000000000000000000000",
//!             "0x000000084F766572666C6F770000000000000000000000000000000000000000",
//!             "0x0000000000000000000000000000000000000000000000000000000000000000"
//!         ],
//!         "exception": true
//!     }, {
//!         "compiler_version": ">=0.8",
//!         "return_data": [
//!             "0x4E487B7100000000000000000000000000000000000000000000000000000000",
//!             "0x0000001100000000000000000000000000000000000000000000000000000000"
//!         ],
//!         "exception": true
//!     } ]
//! }, {
//!     "name": "underflow",
//!     "inputs": [
//!         {
//!             "method": "deposit",
//!             "calldata": [
//!                 "0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe"
//!             ]
//!         },
//!         {
//!             "method": "withdraw",
//!             "calldata": [
//!                 "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
//!             ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [
//!             "0x08c379a000000000000000000000000000000000000000000000000000000000",
//!             "0x0000002000000000000000000000000000000000000000000000000000000000",
//!             "0x00000009556e646572666c6f7700000000000000000000000000000000000000",
//!             "0x0000000000000000000000000000000000000000000000000000000000000000"
//!         ],
//!         "exception": true
//!     }
//! } ] }

// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22;

contract Test {
    uint public balance;
    uint public constant MAX_UINT = 2**256 - 1;

    function deposit(uint _amount) public {
        uint oldBalance = balance;
        uint newBalance = balance + _amount;

        // balance + _amount does not overflow if balance + _amount >= balance
        require(newBalance >= oldBalance, "Overflow");

        balance = newBalance;

        assert(balance >= oldBalance);
    }

    function withdraw(uint _amount) public {
        uint oldBalance = balance;

        // balance - _amount does not underflow if balance >= _amount
        require(balance >= _amount, "Underflow");

        if (balance < _amount) {
            revert("Underflow");
        }

        balance -= _amount;

        assert(balance <= oldBalance);
    }
}
