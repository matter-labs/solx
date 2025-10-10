//! { "cases": [ {
//!     "name": "test",
//!     "inputs": [
//!         {
//!             "method": "test",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "235", "1008", "0", "0", "0",
//!         "86", "4670", "0", "0", "0",
//!         "0", "0", "0", "0", "0",
//!         "0", "0", "0", "0", "0",
//!         "0", "0", "0", "0", "0",
//!         "2", "2"
//!     ]
//! }, {
//!     "name": "fibo",
//!     "inputs": [
//!         {
//!             "method": "fibo",
//!             "calldata": [
//!                 "87"
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "679891637638612258"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.19;
pragma experimental ABIEncoderV2;

contract Test {
    uint16 constant MAX_SIZE = 5;

    struct Matrix {
        uint128[MAX_SIZE][MAX_SIZE] a;
        uint16 n;
        uint16 m;
    }

    function mul(Matrix memory self, Matrix memory other) private pure returns(Matrix memory) {
        Matrix memory result;
        result.n = self.n;
        result.m = other.m;
        for(uint16 i = 0; i < self.n; i++) {
            for(uint16 j = 0; j < other.m; j++) {
                for(uint16 k = 0; k < self.m; k++) {
                    result.a[i][j] += self.a[i][k] * other.a[k][j];
                }
            }
        }
        return result;
    }

    function add(Matrix memory self, Matrix memory other) private pure returns(Matrix memory) {
        for(uint16 i = 0; i < self.n; i++) {
            for(uint16 j = 0; j < self.m; j++) {
                self.a[i][j] += other.a[i][j];
            }
        }
        return self;
    }

    function scalar(Matrix memory self, uint128 n) private pure returns(Matrix memory) {
        for(uint16 i = 0; i < self.n; i++) {
            for(uint16 j = 0; j < self.m; j++) {
                self.a[i][j] *= n;
            }
        }
        return self;
    }

    function pow(Matrix memory self, uint64 n) private pure returns(Matrix memory) {
        Matrix memory result;
        result.n = self.n;
        result.m = self.m;
        for(uint16 i = 0; i < self.n; i++) {
            result.a[i][i] = 1;
        }
        while (n != 0) {
            if (n & 1 != 0) {
                result = mul(result, self);
            }
            self = mul(self, self);
            n >>= 1;
        }
        return result;
    }

    function test() public pure returns(Matrix memory) {
        Matrix memory a;
        a.n = 2;
        a.m = 3;
        a.a[0][0] = 5;
        a.a[0][1] = 2;
        a.a[0][2] = 1;

        a.a[1][0] = 4;
        a.a[1][1] = 2;
        a.a[1][2] = 9;

        a = scalar(a, 2);

        Matrix memory b;
        b.n = 3;
        b.m = 2;
        b.a[0][0] = 1;
        b.a[0][1] = 52;

        b.a[1][0] = 1;
        b.a[1][1] = 2;

        b.a[2][0] = 4;
        b.a[2][1] = 234;

        a = mul(a, b);

        Matrix memory c;
        c.n = 2;
        c.m = 2;
        c.a[0][0] = 213;
        c.a[0][1] = 12;

        c.a[1][0] = 2;
        c.a[1][1] = 34;

        a = add(a, c);
        return a;
    }

    function fibo(uint64 n) public pure returns(uint64) {
        Matrix memory matrix;
        matrix.n = 2;
        matrix.m = 2;
        matrix.a[0][1] = 1;
        matrix.a[1][0] = 1;
        matrix.a[1][1] = 1;
        matrix = pow(matrix, n - 2);
        return uint64(matrix.a[0][1] + matrix.a[1][1]);
    }
}