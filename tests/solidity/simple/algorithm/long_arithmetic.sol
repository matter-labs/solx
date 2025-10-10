//! { "cases": [ {
//!     "name": "test1",
//!     "inputs": [
//!         {
//!             "method": "test1",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1260830800381296"
//!     ]
//! }, {
//!     "name": "test2",
//!     "inputs": [
//!         {
//!             "method": "test2",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "781327317812"
//!     ]
//! }, {
//!     "name": "test3",
//!     "inputs": [
//!         {
//!             "method": "test3",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "0"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.4.19;

contract Test {
    uint64 constant BASE = 1000000000;
    uint16 constant LEN = 12;

    struct BigUint {
        uint64[LEN] digits;
        uint16 len;
    }

    function _new() private pure returns(BigUint memory) {
        BigUint memory bigUint;
        bigUint.len = 1;
        return bigUint;
    }

    function fromUint(uint64 n) private pure returns(BigUint memory) {
        BigUint memory number;
        while (n > 0) {
            number.digits[number.len] = n % BASE;
            n /= BASE;
            number.len += 1;
        }
        if (number.len == 0) {
            number.len = 1;
        }
        return number;
    }

    function asUint(BigUint memory self) private pure returns(uint64) {
        uint64 n = 0;
        uint16 i = self.len - 1;
        while (true) {
            n = n * BASE + self.digits[i];
            if (i == 0) {
                break;
            }
            i--;
        }
        return n;
    }

    function less(BigUint memory self, BigUint memory other) private pure returns(bool) {
        if (self.len != other.len) {
            return self.len < other.len;
        }
        uint16 i = self.len - 1;
        while (true) {
            if (self.digits[i] != other.digits[i]) {
                return self.digits[i] < other.digits[i];
            }
            if (i == 0) {
                break;
            }
            i--;
        }
        return false;
    }
    
    function greater(BigUint memory self, BigUint memory other) private pure returns(bool) {
        return less(other, self);
    }

    function equals(BigUint memory self, BigUint memory other) private pure returns(bool) {
        return !less(self, other) && !less(other, self);
    }

    function add(BigUint memory self, BigUint memory other) private pure returns(BigUint memory) {
        uint16 len = self.len;
        if (other.len > len) {
            len = other.len;
        }
        BigUint memory result;
        result.len = len;
        uint64 carry = 0;
        for (uint16 i = 0; i < len; i++) {
            result.digits[i] += carry;
            if (i < self.len) {
                result.digits[i] += self.digits[i];
            }
            if (i < other.len) {
                result.digits[i] += other.digits[i];
            }
            carry = result.digits[i] / BASE;
            result.digits[i] %= BASE;
        }
        if (carry > 0) {
            result.digits[result.len] = carry;
            result.len++;
        }
        return result;
    }

    // if second greater - 0
    function sub(BigUint memory self, BigUint memory other) private pure returns(BigUint memory) {
        if (less(self, other)) {
            return _new();
        }
        BigUint memory result;
        result.len = self.len;
        uint64 carry = 0;
        for (uint16 i = 0; i < self.len; i++) {
            uint64 d = carry;
            if (i < other.len) {
                d += other.digits[i];
            }
            if (self.digits[i] < d) {
                carry = 1;
                result.digits[i] = self.digits[i] + BASE - d;
            } else {
                carry = 0;
                result.digits[i] = self.digits[i] - d;
            }
        }
        while (result.len > 1 && result.digits[result.len - 1] == 0) {
            result.len--;
        }
        return result;
    }

    function mul(BigUint memory self, BigUint memory other) private pure returns(BigUint memory) {
        BigUint memory result;
        result.len = self.len + other.len;
        for (uint16 i = 0; i < self.len; i++) {
            uint64 carry = 0;
            uint16 j = 0;
            while (j < other.len || carry != 0) {
                uint64 b = 0;
                if (j < other.len) {
                    b = other.digits[j];
                }
                uint64 curr = result.digits[i + j] + self.digits[i] * b + carry;
                result.digits[i + j] = curr % BASE;
                carry = curr / BASE;
                j++;
            }
        }
        while (result.len > 1 && result.digits[result.len - 1] == 0) {
            result.len--;
        }
        return result;
    }

    function div(BigUint memory self, uint64 other) private pure returns(BigUint memory) {
        BigUint memory result;
        result.len = self.len;
        uint64 carry = 0;
        uint16 i = self.len - 1;
        while (true) {
            uint64 curr = self.digits[i] + carry * BASE;
            result.digits[i] = curr / other;
            carry = curr % other;
            if (i == 0) {
                break;
            }
            i--;
        }
        while (result.len > 1 && result.digits[result.len - 1] == 0) {
            result.len--;
        }
        return result;
    }

    function pow(BigUint memory self, uint64 n) private pure returns(BigUint memory) {
        BigUint memory result;
        result.len = 1;
        result.digits[0] = 1;
        while (n != 0) {
            if (n & 1 != 0) {
                result = mul(result, self);
            } 
            self = mul(self, self);
            n >>= 1;
        }
        return result;
    }

    function test1() public pure returns(uint64) {
        BigUint memory number = fromUint(9124712378731);
        number = add(number, fromUint(129213344)); // 9 124 841 592 075
        number = pow(number, 5); // 63 259 633 147 891 755 508 903 200 605 498 033 555 974 923 078 789 924 986 748 046 875
        number = div(number, 987659234); // 64 050 059 950 020 935 569 872 068 452 202 648 627 264 211 938 497 326 889
        number = add(number, fromUint(12332455)); // 64 050 059 950 020 935 569 872 068 452 202 648 627 264 211 938 509 659 344
        number = div(number, 100000000); // 640 500 599 500 209 355 698 720 684 522 026 486 272 642 119 385
        number = sub(number, mul(pow(fromUint(794121), 7), fromUint(3215975))); // 163 633 183 869 615 894 606 020 115 919 720 373 594 410
        number = sub(number, mul(fromUint(46971696617), pow(fromUint(123123), 6))); // 1 307 733 141 469 273 444 912 310 500 297
        number = div(number, 92348823); // 14 160 799 228 261 668 748 201
        number = div(number, 11231324); // 1 260 830 800 381 296
        return asUint(number);
    }

    function test2() public pure returns(uint64) {
        BigUint memory number = fromUint(781327317812);
        number = mul(number, fromUint(8712712));
        number = div(number, 8712712);
        return asUint(number);
    }

    function test3() public pure returns(uint64) {
        BigUint memory number = fromUint(13123121);
        number = sub(number, fromUint(13123122));
        return asUint(number);
    }
}