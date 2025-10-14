//! { "cases": [ {
//!     "name": "decimal",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "0" ]
//!         }
//!     ],
//!     "expected": [
//!         "4024342343240230"
//!     ]
//! }, {
//!     "name": "hexadecimal",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "1" ]
//!         }
//!     ],
//!     "expected": [
//!         "0xdeadbeef"
//!     ]
//! }, {
//!     "name": "true",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "2" ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! }, {
//!     "name": "false",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "3" ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! }, {
//!     "name": "string",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "4" ]
//!         }
//!     ],
//!     "expected": [
//!         "0x6a73646a736a646a6431323331325c2f2212e2888e0000000000000000000000"
//!     ]
//! }, {
//!     "name": "hex",
//!     "inputs": [
//!         {
//!             "method": "#fallback",
//!             "calldata": [ "5" ]
//!         }
//!     ],
//!     "expected": [
//!         "0x0012340000000000000000000000000000000000000000000000000000000000"
//!     ]
//! } ] }

object "Test" {
    code {
        {
            let size := datasize("Test_deployed")
            codecopy(0, dataoffset("Test_deployed"), size)
            return(0, size)
        }
    }

    object "Test_deployed" {
        code {
            {
                switch calldataload(0)
                // decimal
                case 0 {
                    mstore(0, 4024342343240230)
                }
                // hexadecimal
                case 1 {
                    mstore(0, 0x00000000000deadbeef)
                }
                // true
                case 2 {
                    mstore(0, true)
                }
                // false
                case 3 {
                    mstore(0, false)
                }
                // string
                case 4 {
                    mstore(0, "jsdjsjdjd\
12312\\/\"\x12\u220E")
                }
                // hex
                case 5 {
                    mstore(0, hex"001234")
                }
                return(0, 32)
            }
        }
    }
}