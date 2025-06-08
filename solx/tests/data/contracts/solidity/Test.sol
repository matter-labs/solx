// SPDX-License-Identifier: MIT

pragma solidity >=0.5.0;

contract Test {
    function entry() public pure returns(uint64) {
        bool result = true;
        // test returns
        for(uint8 i = 1; i <= 8; i++) {
            result = result && test(i) == i * 10;
        }
        // test break
        for(uint8 i = 1; i <= 8; i++) {
            result = result && test(20 + i) == 0;
        }
        // test continue
        result = result && test(11) == 0;
        result = result && test(12) == 0;
        result = result && test(13) == 0;

        // cnt
        result = result && test(0) == 864;

        result = result && test(31) == 288;
        result = result && test(32) == 576;
        result = result && test(33) == 216;

        if (result) {
            return 1;
        } else {
            return 0;
        }
    }

    function test(uint8 param) private pure returns(uint64) {
        uint64 cnt = 0;

        for(uint8 i = 0; i < 2; i++) {
            if (param == 1) {
                return param * 10; // 10
            }
            if (param == 21) {
                break;
            }
            if (param == 11) {
                continue;
            }
            uint8 j = 1;
            while(j < 4) {
                uint8 p = 0;
                if (param == 22) {
                    break;
                }
                if (param == 33 && j > 1) {
                    return cnt;
                }
                do {
                    if (param == 3) {
                        return param * 10; // 30
                    }
                    p += 2;
                    if (p % 8 == 0) {
                        break;
                    }
                    if (param == 12) {
                        continue;
                    }
                    if (param == 23) {
                        break;
                    }
                    for(uint8 h = 1; h <= 4; h++) {
                        if (param == 4) {
                            return 40;
                        }
                        if (h > 2) {
                            break;
                        }
                        if (param == 24) {
                            break;
                        }
                        for(uint8 k = 10; k < 12; k++) {
                            uint8 x = 7;
                            if (param == 5) {
                                return 5 * k;
                            }
                            if (param == 25) {
                                break;
                            }
                            do {
                                x -= 1;
                                if (x == 0) {
                                    break;
                                }
                                if (x % 2 == 0) {
                                    continue;
                                }
                                uint8 y = 10;
                                if (param == 26) {
                                    break;
                                }
                                while(y < 17) {
                                    if (param == 7) {
                                        return 7 * y; // 70
                                    }
                                    if (param == 27) {
                                        break;
                                    }
                                    y += 1;
                                    if (y % 3 == 0) {
                                        continue;
                                    }
                                    if (y == 16) {
                                        break;
                                    }
                                    if (param == 31 && y > 11) {
                                        continue;
                                    }
                                    if (param == 32 && y > 13) {
                                        break;
                                    }
                                    for(uint8 l = 0; l < 4; l++) {
                                        if (param == 28) {
                                            break;
                                        }
                                        if (l % 2 == 0) {
                                            continue;
                                        }
                                        if (param == 8) {
                                            return 80;
                                        }
                                        if (param == 13) {
                                            continue;
                                        }
                                        cnt += 1;
                                    }
                                }
                                if (param == 6) {
                                    return 60;
                                }
                            } while(true);
                        }
                    }
                } while(true);
                j *= 2;
                if (param == 2) {
                    return 20;
                }
            }
        }
        
        return cnt;
    }
}
