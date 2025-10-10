//! { "cases": [ {
//!     "name": "simple",
//!     "inputs": [
//!         {
//!             "method": "simple",
//!             "calldata": [
//!                 "10"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "10"
//!     ]
//! }, {
//!     "name": "complex",
//!     "inputs": [
//!         {
//!             "method": "complex",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.16;

contract Test {
    function malloc(uint24 size) private pure returns(uint24 returnPointer) {
        assembly {
            returnPointer := mload(64)
            mstore(64, add(returnPointer, mul(size, 32)))
        }
    }

    struct Vec {
        uint24 ptr;
        uint24 cap;
        uint24 len;
    }

    function grow(Vec memory self) private pure returns(Vec memory) {
        uint24 newCap = 0;
        if (self.cap == 0) {
            newCap = 1;
        } else {
            newCap = self.cap * 2;
        }
        uint24 newPtr = malloc(newCap);
        for (uint24 i = 0; i < self.len; i++) {
            assembly {
                mstore(add(newPtr, mul(i, 32)), mload(add(mload(self), mul(i, 32))))
            }
        }
        self.ptr = newPtr;
        self.cap = newCap;
        return self;
    }

    function _new() private pure returns(Vec memory) {
        return Vec(0, 0, 0);
    }

    function insert(Vec memory self, uint24 index, uint64 val) private pure returns(Vec memory) {
        if (self.cap == self.len) {
            grow(self);
        }

        uint24 curr = self.len;
        while (curr > index) {
            assembly {
                mstore(add(mload(self), mul(curr, 32)), mload(sub(add(mload(self), mul(curr, 32)), 32)))
            }
            curr -= 1;
        }
        assembly {
            mstore(add(mload(self), mul(index, 32)), val)
        }

        self.len += 1;

        return self;
    }

    function remove(Vec memory self, uint24 index) private pure returns(Vec memory) {
        self.len--;
        while (index < self.len) {
            assembly {
                mstore(add(mload(self), mul(index, 32)), mload(add(add(mload(self), mul(index, 32)), 32)))
            }
            index += 1;
        }
        assembly {
            mstore(add(mload(self), mul(mload(add(self, 0x40)), 32)), 0)
        }
        
        return self;
    }

    function push(Vec memory self, uint64 val) private pure returns(Vec memory) {
        return insert(self, self.len, val);
    }

    function pop(Vec memory self) private pure returns(Vec memory) {
        return remove(self, self.len - 1);
    }

    function get(Vec memory self, uint24 index) private pure returns(uint64 result) {
        assembly {
            result := mload(add(mload(self), mul(index, 32)))
        }
    }

    function simple(uint64 value) public pure returns(uint64) {
        Vec memory vec = _new();
        push(vec, value);
        return get(vec, 0);
    }

    function complex() public pure returns(uint64) {
        Vec memory vec = _new();
        bool result = true;

        for(uint8 _i = 0; _i < 100; _i++) {
            push(vec, _i);
        }

        //0 - 99
        if (vec.len != 100) {
            result = false;
        }

        uint24 i = 98;
        while (true) {
            remove(vec, i);
            if (i == 0) {
                break;
            }
            i -= 2;
        }
        // 1, 3, 5, 7,..., 99

        if (vec.len != 50) {
            result = false;
        }
        while (vec.len > 25) {
            pop(vec);
        }
        // 1, 3, 5, 7,..., 49

        if (vec.len != 25) {
            result = false;
        }
        i = 0;
        while (i < 50) {
            insert(vec, i, i);
            i += 2;
        }

        if (vec.len != 50) {
            result = false;
        }

        for(uint8 ind = 0; ind < 50; ind++) {
            if (get(vec, ind) != ind) {
                result = false;
            }
        }

        if (result) {
            return 1;
        } else {
            return 0;
        }
    } 
}