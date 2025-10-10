//! { "cases": [ {
//!     "name": "distancePoint1",
//!     "inputs": [
//!         {
//!             "method": "distancePointEntry",
//!             "calldata": [
//!                 "7", "4", "10",
//!                 "5", "1"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "607"
//!     ]
//! }, {
//!     "name": "distancePoint2",
//!     "inputs": [
//!         {
//!             "method": "distancePointEntry",
//!             "calldata": [
//!                 "2117", "443", "1021",
//!                 "5123", "145"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "504455"
//!     ]
//! }, {
//!     "name": "intersectionPoint1",
//!     "inputs": [
//!         {
//!             "method": "intersectionPoint",
//!             "calldata": [
//!                 "1", "1", "0",
//!                 "1", "1", "2"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "100", "100"
//!     ]
//! }, {
//!     "name": "intersectionPoint2",
//!     "inputs": [
//!         {
//!             "method": "intersectionPoint",
//!             "calldata": [
//!                 "121", "17", "123",
//!                 "912", "123", "2133"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "69", "1218"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.19;
pragma experimental ABIEncoderV2;

contract Test {
    uint128 constant EPS = 1E6;
    uint128 constant PRECISION = 100;

    struct Point {
        uint128 x;
        uint128 y;
    }

    struct Line {
        uint128 a;
        uint128 b;
        uint128 c;
    }

    function distancePointEntry(Line memory line, Point memory point) public pure returns(uint64) {
        // 6.077701994871213... * PRECISION
        return uint64(distancePoint(line, point) * PRECISION / EPS);
    }

    function intersectionPoint(uint128 a1, uint128 mb1, uint128 c1, uint128 a2, uint128 b2, uint128 mc2) public pure returns(Point memory) {
        Point memory point = linesIntersection(a1, mb1, c1, a2, b2, mc2);
        point.x *= PRECISION;
        point.x /= EPS;
        point.y *= PRECISION;
        point.y /= EPS;
        return point;
    }

    function sqrt(uint128 n) private pure returns(uint128) {
        // binary search
        uint128 l = 0;
        uint128 r = n;
        while (l < r) {
            uint128 m = (l + r + 1) / 2;
            if (m*m <= n) {
                l = m;
            } else {
                r = m - 1;
            }
        }
        if (n - l*l < (l+1)*(l+1) - n) {
            return l;
        } else {
            return l + 1;
        }
    }

    // mb1 == -b1, mc2 == -c2
    // returns x * EPS, y * EPS
    function linesIntersection(uint128 a1, uint128 mb1, uint128 c1, uint128 a2, uint128 b2, uint128 mc2) private pure returns(Point memory) {
        uint128 d = a1 * b2 + a2 * mb1;
        return Point((mb1 * mc2 - b2 * c1) * EPS / d, (a2 * c1 + a1 * mc2) * EPS / d);
    }

    function distancePoint(Line memory line, Point memory point) private pure returns(uint128) {
        return ((line.a * point.x + line.b * point.y + line.c) * EPS * EPS) / sqrt((line.a * line.a + line.b * line.b) * EPS * EPS);
    }
}