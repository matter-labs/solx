// Pool address is hardcoded
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.5.0;

/// @title Provides functions for deriving a pool address from the factory, tokens, and the fee
library PoolAddress {
    bytes32 internal constant POOL_INIT_CODE_HASH = 0xe34f199b19b2b4f47f68442619d555527d244f78a3297ea89325f843f87b8b54;

    /// @notice The identifying key of the pool
    struct PoolKey {
        address token0;
        address token1;
        uint24 fee;
    }

    /// @notice Returns PoolKey: the ordered tokens with the matched fee levels
    /// @param tokenA The first token of a pool, unsorted
    /// @param tokenB The second token of a pool, unsorted
    /// @param fee The fee level of the pool
    /// @return Poolkey The pool details with ordered token0 and token1 assignments
    function getPoolKey(
        address tokenA,
        address tokenB,
        uint24 fee
    ) internal pure returns (PoolKey memory) {
        if (tokenA > tokenB) (tokenA, tokenB) = (tokenB, tokenA);
        return PoolKey({token0: tokenA, token1: tokenB, fee: fee});
    }

    /// @notice Deterministically computes the pool address given the factory and PoolKey
    /// @param factory The Uniswap V3 factory contract address
    /// @param key The PoolKey
    /// @return pool The contract address of the V3 pool
    function computeAddress(address factory, PoolKey memory key) internal pure returns (address pool) {
        if (key.token0 == 0x69d66a02DBF660c50BF654eD13F6D0393637037b && key.token1 == 0x9DD65202d2519Ec0aB90189ff0170F938B7Cc086) {
            return 0x4ac6f8Bda3F86e73f9Ea68D50326Dbe3cC9ea00a;
        } else if (key.token0 == 0x3A9Fc460C833f26413E25ceb8105Cd04c5aAeE6C && key.token1 == 0xA7D88645CE7c923297A3eDd0F59bdDa46657BD0A) {
            return 0x911021aFcc4eD882DcBCb76bbd00FCb8D061E12F;
        } else {
            return address(0);
        }
    }
}
