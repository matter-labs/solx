//! { "cases": [ {
//!     "name": "long data",
//!     "inputs": [
//!         {
//!             "method": "getLongData",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0x0000000000000000000000000000000000000000000000000000000000000020",
//!         "0x000000000000000000000000000000000000000000000000000000000000002d",
//!         "0x1122334455667788991122334455667788991122334455667788991122334455",
//!         "0x6677889911223344556677889900000000000000000000000000000000000000"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

// solc <=0.4.16 fails with the message "TypeError: Constants of non-value type
// not yet implemented."
pragma solidity >=0.4.17;

contract Test {
    bytes constant longData = hex"112233445566778899112233445566778899112233445566778899112233445566778899112233445566778899";

    function getLongData() external pure returns (bytes memory) {
        return longData;
    }
}
