//! { "cases": [ {
//!     "name": "complex",
//!     "inputs": [
//!         {
//!             "method": "get",
//!             "calldata": [
//!             ],
//!              "expected": [
//!                 "0"
//!             ]
//!         }, {
//!             "method": "set",
//!             "calldata": [
//!                 "1"
//!             ]
//!         }, {
//!             "method": "status",
//!             "calldata": [
//!             ],
//!              "expected": [
//!                 "1"
//!             ]
//!         }, {
//!             "method": "cancel",
//!             "calldata": [
//!             ]
//!         }, {
//!             "method": "get",
//!             "calldata": [
//!             ],
//!              "expected": [
//!                 "4"
//!             ]
//!         }, {
//!             "method": "set",
//!             "calldata": [
//!                 "3"
//!             ]
//!         }, {
//!             "method": "get",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "3"
//!     ]
//! }, {
//!     "name": "invalidEnumValue",
//!     "inputs": [
//!         {
//!             "method": "set",
//!             "calldata": [
//!                 "5"
//!             ]
//!         }
//!     ],
//!     "expected": {
//!         "return_data": [],
//!         "exception": true
//!     }
//! } ] }

// These are two different situations: Conversion failure from integer to enum in user-supplied code results
// in Panic(0x21), while ABI-decoding inputs is not a user-supplied conversion: We have to expect that input data is
// invalid and thus the ABI decoding code only uses Error() reverts or their "shorter" version
// (when --revert-strings is set to default) where the error data is completely empty. On the other hand,
// if you forcefully convert an integer to an enum in user-code, the idea is that you already checked that the
// conversion will work out and thus avoid a panic error.
//
// https://linear.app/matterlabs/issue/CPR-370/empty-error-for-invalid-enum-value
// https://github.com/ethereum/solidity/issues/12368

// SPDX-License-Identifier: MIT
pragma solidity >=0.4.16;

contract Test {
    // Enum representing shipping status
    enum Status {
        Pending,
        Shipped,
        Accepted,
        Rejected,
        Canceled
    }

    // Default value is the first element listed in
    // definition of the type, in this case "Pending"
    Status public status;

    // Returns uint
    // Pending  - 0
    // Shipped  - 1
    // Accepted - 2
    // Rejected - 3
    // Canceled - 4
    function get() public view returns (Status) {
        return status;
    }

    // Update status by passing uint into input
    function set(Status _status) public {
        status = _status;
    }

    // You can update to a specific enum like this
    function cancel() public {
        status = Status.Canceled;
    }

    // delete resets the enum to its first value, 0
    function reset() public {
        delete status;
    }
}
