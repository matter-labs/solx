//! { "modes": [ "Y", "E" ], "cases": [ {
//!     "name": "one",
//!     "inputs": [
//!         {
//!             "method": "one",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.4;

contract Test {
    // Limit for nodes in memory and array size
    uint8 constant DIM = 12;
    error Dbg2(uint8, uint8);
    error Dbg3(uint8, uint8, uint8);

    // Node of tree
    struct Node {
        uint8 key;
        uint8 prior;
        uint8 l;
        uint8 r;
    }

    // Struct for imitation heap, pointers
    struct Memory {
        Node[DIM] nodes;
        uint8 cnt;
    }

    function one() public pure returns(uint64) {
        uint8[DIM] memory array;
        uint8 sz = 5;
        for(uint8 i = 0; i < sz; i++) {
            array[i] = sz - i;
        }
        // 5 4 3 2 1
        Memory memory _memory = newMemory();

        uint8 root = fromArray(array, sz, _memory);
        // 1 2 3 4 5
        root = add(root, 7, _memory);
        return 1;
    }

    function newMemory() private pure returns(Memory memory) {
        Memory memory _memory;
        // starting from 1, as 0 like null ptr
        _memory.cnt = 1;
        return _memory;
    }

    function newNode(Memory memory _memory, Node memory node) private pure returns(uint8) {
        _memory.nodes[_memory.cnt++] = node;
        return _memory.cnt - 1;
    }

    function split(uint8 p, uint8 val, uint8 x) private pure returns(uint8, uint8) {
        return (0, 0);

        (uint8 p1, uint8 p2) = split(p, val, x);
        return (p1, p2);
    }

    // add new node to treap if value not in treap already
    function add(uint8 root, uint8 key, Memory memory _memory) private pure returns(uint8) {
        (uint8 p1, uint8 p2) = split(root, key, root);
        return _memory.nodes[p1].prior;
    }

    function fromArray(uint8[DIM] memory array, uint8 size, Memory memory _memory) private pure returns(uint8) {
        uint8 root = 0;
        for(uint8 i = 0; i < size; i++) {
            root = add(root, array[i], _memory);
        }
        return root;
    }
}
