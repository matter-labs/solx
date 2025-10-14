//! { "cases": [ {
//!     "name": "simple",
//!     "inputs": [
//!         {
//!             "method": "simple",
//!             "calldata": [
//!                 "10", "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
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

pragma solidity >=0.5.11;

// Hash map implementation with vector
contract Test {
    function malloc(uint24 size) private pure returns(uint24 returnPointer) {
        assembly {
            returnPointer := mload(64)
            mstore(64, add(returnPointer, mul(size, 32)))
        }
    }

    // Map entry
    struct Entry {
        uint64 key;
        uint64 value;
    }

    uint24 constant TYPE_SIZE = 2;

    // Vector implementation
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
        uint24 newPtr = malloc(newCap * TYPE_SIZE);
        for (uint24 i = 0; i < self.len; i++) {
            assembly {
                mstore(add(newPtr, mul(mul(i, TYPE_SIZE), 32)), mload(add(mload(self), mul(mul(i, TYPE_SIZE), 32))))
                mstore(add(add(newPtr, mul(mul(i, TYPE_SIZE), 32)), 32), mload(add(add(mload(self), mul(mul(i, TYPE_SIZE), 32)), 32)))
            }
        }
        self.ptr = newPtr;
        self.cap = newCap;
        return self;
    }

    function _new() private pure returns(Vec memory) {
        return Vec(0, 0, 0);
    }

    function insert(Vec memory self, uint24 index, Entry memory val) private pure returns(Vec memory) {
        if (self.cap == self.len) {
            grow(self);
        }

        uint24 curr = self.len;
        while (curr > index) {
            assembly {
                mstore(add(mload(self), mul(mul(curr, TYPE_SIZE), 32)), mload(sub(add(mload(self), mul(mul(curr, TYPE_SIZE), 32)), mul(32, TYPE_SIZE))))
                mstore(add(add(mload(self), mul(mul(curr, TYPE_SIZE), 32)), 32), mload(add(sub(add(mload(self), mul(mul(curr, TYPE_SIZE), 32)), mul(32, TYPE_SIZE)), 32)))
            }
            curr -= 1;
        }
        assembly {
            mstore(add(mload(self), mul(mul(index, TYPE_SIZE), 32)), mload(val))
            mstore(add(add(mload(self), mul(mul(index, TYPE_SIZE), 32)), 32), mload(add(val, 32)))
        }

        self.len += 1;

        return self;
    }

    function remove(Vec memory self, uint24 index) private pure returns(Vec memory) {
        self.len--;
        while (index < self.len) {
            assembly {
                mstore(add(mload(self), mul(mul(index, TYPE_SIZE), 32)), mload(add(add(mload(self), mul(mul(index, TYPE_SIZE), 32)), mul(32, TYPE_SIZE))))
                mstore(add(add(mload(self), mul(mul(index, TYPE_SIZE), 32)), 32), mload(add(add(add(mload(self), mul(mul(index, TYPE_SIZE), 32)), mul(32, TYPE_SIZE)), 32)))
            }
            index += 1;
        }
        assembly {
            mstore(add(mload(self), mul(mul(mload(add(self, 0x40)), TYPE_SIZE), 32)), 0)
            mstore(add(add(mload(self), mul(mul(mload(add(self, 0x40)), TYPE_SIZE), 32)), 32), 0)
        }
        
        return self;
    }

    function push(Vec memory self, Entry memory val) private pure returns(Vec memory) {
        return insert(self, self.len, val);
    }

    function pop(Vec memory self) private pure returns(Vec memory) {
        return remove(self, self.len - 1);
    }

    function get(Vec memory self, uint24 index) private pure returns(Entry memory result) {
        assembly {
            mstore(result, mload(add(mload(self), mul(mul(index, TYPE_SIZE), 32))))
            mstore(add(result, 32), mload(add(add(mload(self), mul(mul(index, TYPE_SIZE), 32)), 32)))
        }
    }

    uint24 constant CAP = 32;

    // Hash map implementation
    struct HashMap {
        Vec[CAP] table;
        uint24 len;
    }

    function newMap() private pure returns(HashMap memory) {
        HashMap memory result;
        for(uint24 i = 0; i < CAP; i++) {
            result.table[i] = _new();
        }
        return result;
    }

    function contains_key(HashMap memory self, uint64 key) private pure returns(bool) {
        uint64 hash = key % CAP;
        for(uint24 i = 0; i < self.table[hash].len; i++) {
            if (get(self.table[hash], i).key == key) {
                return true;
            }
        }
        return false;
    }

    function insert(HashMap memory self, uint64 key, uint64 value) private pure returns(HashMap memory) {
        if (contains_key(self, key)) {
            return self;
        }
        self.len += 1;
        uint64 hash = key % CAP;
        push(self.table[hash], Entry(key, value));
        return self;
    }

    function remove(HashMap memory self, uint64 key) private pure returns(HashMap memory) {
        uint64 hash = key % CAP;
        uint24 i = 0;
        while (i < self.table[hash].len) {
            if (get(self.table[hash], i).key == key) {
                remove(self.table[hash], i);
                self.len -= 1;
                break;
            }
            i += 1;
        }
        return self;
    }

    function get(HashMap memory self, uint64 key) private pure returns(uint64) {
        uint64 hash = key % CAP;
        for (uint24 i = 0; i < self.table[hash].len; i++) {
            if (get(self.table[hash], i).key == key) {
                return get(self.table[hash], i).value;
            }
        }
        return 0;
    }

    function simple(uint64 key, uint64 value) public pure returns(uint64) {
        HashMap memory map = newMap();
        insert(map, key, value);
        return get(map, key);
    }

    function complex() public pure returns(uint64) {
        HashMap memory map = newMap();
        for(uint8 _i = 0; _i < 5; _i++) {
            insert(map, _i, 5 - _i);
        }
        // 0 - 4
        if (map.len != 5) {
            return 0;
        }
        uint64 i = 0;
        while (i < 5) {
            remove(map, i);
            i += 2;
        }
        // 1, 3
        if (map.len != 2) {
            return 0;
        }
        i = 0;
        while (i < 5) {
            insert(map, i, 5 - i);
            i += 3;
        }
        // 0, 1, 3
        if (map.len != 3) {
            return 0;
        }
        bool result = true;
        for (uint8 key = 0; key < 5; key++) {
            if (key % 2 == 1 || key % 3 == 0) {
                result = result && contains_key(map, key);
                result = result && get(map, key) == 5 - key;
            } else {
                result = result && !contains_key(map, key);
            }
        }
        if (result) {
            return 1;
        } else {
            return 0;
        }
    }
}