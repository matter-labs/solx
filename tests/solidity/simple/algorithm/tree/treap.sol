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
//! }, {
//!     "name": "two",
//!     "inputs": [
//!         {
//!             "method": "two",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

// solc < 0.5.0 complains "Error: Identifier already declared. uint8 p = ..."
pragma solidity >=0.5.0;

contract Test {
    // Limit for nodes in memory and array size
    uint8 constant DIM = 12;

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
        // 1 2 3 4 5 7
        root = erase(root, 3, _memory);
        // 1 2 4 5 7
        root = add(root, 0, _memory);
        //0 1 2 4 5 7
        uint8 resSz = toArray(root, _memory, array);
        bool result = resSz == 6 &&
            array[0] == 0 &&
            array[1] == 1 &&
            array[2] == 2 &&
            array[3] == 4 &&
            array[4] == 5 &&
            array[5] == 7;
        if (result) {
            return 1;
        } else {
            return 0;
        }
    }

    function two() public pure returns(uint64) {
        Memory memory _memory = newMemory();
        uint8 root = 0;

        uint8 i = 10;
        while(i > 0) {
            root = add(root, i, _memory);
            i -= 2;
        }
        // 2 4 6 8 10

        i = 1;
        while (i <= 9) {
            root = add(root, i, _memory);
            i += 2;
        }
        // 1 - 10

        i = 3;
        while (i <= 9) {
            root = erase(root, i, _memory);
            i += 3;
        }
        // 1 2 4 5 7 8 10

        i = 4;
        while (i <= 8) {
            root = erase(root, i, _memory);
            i += 4;
        }
        // 1 2 5 7 10

        root = add(root, 6, _memory);
        // 1 2 5 6 7 10

        uint8[DIM] memory array;
        uint8 sz = toArray(root, _memory, array);

        bool result = sz == 6 &&
            array[0] == 1 &&
            array[1] == 2 &&
            array[2] == 5 &&
            array[3] == 6 &&
            array[4] == 7 &&
            array[5] == 10;
        if (result) {
            return 1;
        } else {
            return 0;
        }
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

    // merge and split for treap
    function merge(uint8 p1, uint8 p2, Memory memory _memory) private pure returns(uint8) {
        if (p1 == 0) {
            return p2;
        }
        if( p2 == 0) {
            return p1;
        }
        if(_memory.nodes[p1].prior > _memory.nodes[p2].prior) {
            uint8 p = merge(_memory.nodes[p1].r, p2, _memory);
            _memory.nodes[p1].r = p;
            return p1;
        } else {
            uint8 p = merge(p1, _memory.nodes[p2].l, _memory);
            _memory.nodes[p2].l = p;
            return p2;
        }
    }

    function split(uint8 p, uint8 val, Memory memory _memory) private pure returns(uint8, uint8) {
        if (p == 0) {
            return (0, 0);
        }
        if (_memory.nodes[p].key < val) {
            (uint8 p1, uint8 p2) = split(_memory.nodes[p].r, val, _memory);
            _memory.nodes[p].r = p1;
            return (p, p2);
        } else {
            (uint8 p1, uint8 p2) = split(_memory.nodes[p].l, val, _memory);
            _memory.nodes[p].l = p2;
            return (p1, p);
        }
    }

    // add new node to treap if value not in treap already
    function add(uint8 root, uint8 key, Memory memory _memory) private pure returns(uint8) {
        (uint8 p1, uint8 p2) = split(root, key, _memory);
        // if key already in tree - ignore
        (uint8 tmp1, uint8 tmp2) = split(p2, key + 1, _memory);
        p2 = merge(tmp1, tmp2, _memory);
        if (tmp1 != 0) {
            return merge(p1, p2, _memory);
        }

        uint8 _new = newNode(_memory, Node(
            key,
            // instead random
            _memory.cnt * key,
            0,
            0
        ));

        return merge(merge(p1, _new, _memory), p2, _memory);
    }

    // remove key from treap, if it containing
    function erase(uint8 root, uint8 key, Memory memory _memory) private pure returns(uint8) {
        (uint8 p1, uint8 p2) = split(root, key, _memory);
        (uint8 p21, uint8 p22) = split(p2, key + 1, _memory);
        return merge(p1, p22, _memory);
    }

    function fromArray(uint8[DIM] memory array, uint8 size, Memory memory _memory) private pure returns(uint8) {
        uint8 root = 0;
        for(uint8 i = 0; i < size; i++) {
            root = add(root, array[i], _memory);
        }
        return root;
    }

    // works so long, as with every call creates temporary array DIM size
    function toArray(uint8 root, Memory memory _memory, uint8[DIM] memory array) private pure returns(uint8) {
        if (root == 0) {
            return 0;
        }
        uint8 lSize = toArray(_memory.nodes[root].l, _memory, array);
        uint8[DIM] memory tmp;
        uint8 rSize = toArray(_memory.nodes[root].r, _memory, tmp);
        uint8 sz = lSize;
        array[sz++] = _memory.nodes[root].key;
        for(uint8 i = 0; i < rSize; i++) {
            array[sz++] = tmp[i];
        }
        return sz;
    }
}
