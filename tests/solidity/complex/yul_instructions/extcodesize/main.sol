// SPDX-License-Identifier: MIT

pragma solidity >=0.5.0;

contract Main {
    constructor() public {
        assembly {
            mstore(0, gt(extcodesize(address()), 0))
            log0(0, 32)
        }
    }

    function main(uint256 _address) external view {
        assembly {
            mstore(0, gt(extcodesize(_address), 0))
            return(0, 32)
        }
    }

    function dirty_address_test(address _address) external view {
        uint256 dirty_address = uint256(uint160(_address)) | (0xff001122deadbeef9944eeaa << 160);
        assembly {
            mstore(0, gt(extcodesize(dirty_address), 0))
            return(0, 32)
        }
    }
}
