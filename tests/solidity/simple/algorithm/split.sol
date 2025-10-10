//! { "modes": [ "Y", "E" ], "cases": [ {
//!     "name": "default",
//!     "inputs": [
//!         {
//!             "method": "f",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!       "0",
//!       "0",
//!       "0",
//!       "3",
//!       "0",
//!       "0",
//!       "2",
//!       "0",
//!       "1"
//!     ]
//! } ] }

// Report https://linear.app/matterlabs/issue/CPR-586/treap-minimization-failure-for-m1-m3

// SPDX-License-Identifier: MIT

pragma solidity >=0.5.0;
pragma experimental ABIEncoderV2;

contract Test {
    struct Node {
        uint256 key;
        uint256 l;
        uint256 r;
    }

    function f() public pure returns(Node[3] memory) {
        Node[3] memory nodes;
        nodes[1] = Node(3, 0, 0);
        nodes[2] = Node(2, 0, 1);

        split(2, nodes);

        return nodes;
    }

    function split(uint256 p, Node[3] memory nodes) private pure returns(uint256, uint256) {
        if (p == 0) {
            return (0, 0);
        }
        if (nodes[0].key < 1) {
            (uint256 p1, uint256 p2) = split(nodes[p].r, nodes);
            nodes[p].r = p1;
            return (p, 0);
        } else { // Should be unreachable(nodes[0].key == 0 always), but if remove this branch - test will work.
            (uint256 p1, uint256 p2) = split(nodes[p].l, nodes);
            nodes[p].l = p2;
            return (0, 0);
        }
    }
}