// SPDX-License-Identifier: MIT

pragma solidity >=0.6.0;

contract Main {
    function main(uint256 _address) external payable {
        assembly {
            let result := call(gas(), _address, callvalue(), 0, 0, 0, 0)
            mstore(0, result)
            mstore(32, balance(_address))
            return(0, 64)
        }
    }

    function dirty_address_test(address _address) external {
        uint256 _address = uint256(uint160(_address)) | (0xff001122deadbeef9944eeaa << 160);
        assembly {
            mstore(0, balance(_address))
            return(0, 32)
        }
    }

    fallback() external payable {}
}
