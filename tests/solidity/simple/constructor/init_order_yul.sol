//! { "modes": [ "Y", "I" ], "cases": [ {
//!     "name": "outerConstructedStartTokenId",
//!     "inputs": [
//!         {
//!             "method": "innerConstructedStartTokenId",
//!             "calldata": []
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

// Fails with the Yul pipeline even with the latest solc (v0.8.19).
// Works only with EVM legacy assembly.
// The problem is illustrated with the Yul example below. It is probably in the Yul inliner that violates the order of the statements:
//
// sstore(/** @src -1:-1:-1 */ 0, /** @src 0:816:1267  "contract Test is InnerContract {..." */ sload(/** @src 0:1132:1146  "START_TOKEN_ID" */ 0x02)) // read before write
// sstore(/** @src 0:1132:1146  "START_TOKEN_ID" */ 0x02, /** @src 0:934:935  "1" */ 0x01) // write after read
// sstore(/** @src 0:934:935  "1" */ 0x01, 0x01)

pragma solidity >=0.8.0;

contract InnerContract {
    uint256 public innerConstructedStartTokenId;

    constructor() {
        innerConstructedStartTokenId = _startTokenId();
    }

    function _startTokenId() internal view virtual returns (uint256) {
        return 0;
    }

    function innerStartTokenId() public view virtual returns (uint256) {
        return _startTokenId();
    }
}

contract Test is InnerContract {
    uint256 public outerConstructedStartTokenId;
    uint256 public START_TOKEN_ID = 1;

    constructor() InnerContract() {
        outerConstructedStartTokenId = _startTokenId();
    }

    function _startTokenId() internal view virtual override returns (uint256) {
        return START_TOKEN_ID;
    }

    function outerStartTokenId() public view virtual returns (uint256) {
        return _startTokenId();
    }
}
