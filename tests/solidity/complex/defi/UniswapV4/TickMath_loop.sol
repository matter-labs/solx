// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {TickMath} from "./lib/libraries/TickMath.sol";

contract TickMathLoop {
    function getSqrtPriceAtTick(int24 tick) public pure returns(uint160) {
      return TickMath.getSqrtPriceAtTick(tick);
    }
    function getTickAtSqrtPrice(uint160 sqrtPriceX96) public pure returns (int24) {
      return TickMath.getTickAtSqrtPrice(sqrtPriceX96);
    }
    function getTickAtSqrtPrice_sol_loop(uint160 sqrtPriceX96) public pure returns (int24) {
      return TickMath.getTickAtSqrtPrice_sol_loop(sqrtPriceX96);
    }
    function getTickAtSqrtPrice_yul_loop(uint160 sqrtPriceX96) public pure returns (int24) {
      return TickMath.getTickAtSqrtPrice_yul_loop(sqrtPriceX96);
    }
}
