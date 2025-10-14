pragma solidity =0.7.6;

import './v3-core/interfaces/IUniswapV3Pool.sol';

contract _InitWrapper {
    function initialize(address pool, uint160 sqrtPriceX96) external {
        IUniswapV3Pool(pool).initialize(sqrtPriceX96);
    }
}
