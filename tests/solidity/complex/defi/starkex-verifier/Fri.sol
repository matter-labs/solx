/*
  Copyright 2019-2022 StarkWare Industries Ltd.

  Licensed under the Apache License, Version 2.0 (the "License").
  You may not use this file except in compliance with the License.
  You may obtain a copy of the License at

  https://www.starkware.co/open-source-license/

  Unless required by applicable law or agreed to in writing,
  software distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions
  and limitations under the License.
*/
// SPDX-License-Identifier: Apache-2.0.
pragma solidity ^0.6.12;

import "./cpu/layout_dydx/MemoryMap.sol";
import "./MemoryAccessUtils.sol";
import "./FriLayer.sol";
import "./HornerEvaluator.sol";

/*
  This contract computes and verifies all the FRI layer, one by one. The final layer is verified
  by evaluating the fully committed polynomial, and requires specific handling.
*/
contract Fri is MemoryMap, MemoryAccessUtils, HornerEvaluator, FriLayer {
    function verifyLastLayer(uint256[] memory ctx, uint256 nPoints) private view {
        uint256 friLastLayerDegBound = ctx[MM_FRI_LAST_LAYER_DEG_BOUND];
        uint256 groupOrderMinusOne = friLastLayerDegBound * ctx[MM_BLOW_UP_FACTOR] - 1;
        uint256 coefsStart = ctx[MM_FRI_LAST_LAYER_PTR];

        for (uint256 i = 0; i < nPoints; i++) {
            uint256 point = ctx[MM_FRI_QUEUE + FRI_QUEUE_SLOT_SIZE * i + 2];
            // Invert point using inverse(point) == fpow(point, ord(point) - 1).

            point = fpow(point, groupOrderMinusOne);
            require(
                hornerEval(coefsStart, point, friLastLayerDegBound) ==
                ctx[MM_FRI_QUEUE + FRI_QUEUE_SLOT_SIZE * i + 1],
                "Bad Last layer value."
            );
        }
    }

    /*
      Verifies FRI layers.

      See FriLayer for the descriptions of the FRI context and FRI queue.
    */
    function friVerifyLayers(uint256[] memory ctx) internal view virtual {
        uint256 friCtx = getPtr(ctx, MM_FRI_CTX);
        require(
            MAX_SUPPORTED_FRI_STEP_SIZE == FRI_MAX_STEP_SIZE,
            "MAX_STEP_SIZE is inconsistent in MemoryMap.sol and FriLayer.sol"
        );
        initFriGroups(friCtx);
        uint256 channelPtr = getChannelPtr(ctx);
        uint256 merkleQueuePtr = getMerkleQueuePtr(ctx);

        uint256 friStep = 1;
        uint256 nLiveQueries = ctx[MM_N_UNIQUE_QUERIES];

        // Add 0 at the end of the queries array to avoid empty array check in readNextElment.
        ctx[MM_FRI_QUERIES_DELIMITER] = 0;

        // Rather than converting all the values from Montgomery to standard form,
        // we can just pretend that the values are in standard form but all
        // the committed polynomials are multiplied by MontgomeryR.
        //
        // The values in the proof are already multiplied by MontgomeryR,
        // but the inputs from the OODS oracle need to be fixed.
        for (uint256 i = 0; i < nLiveQueries; i++) {
            ctx[MM_FRI_QUEUE + FRI_QUEUE_SLOT_SIZE * i + 1] = fmul(
                ctx[MM_FRI_QUEUE + FRI_QUEUE_SLOT_SIZE * i + 1],
                K_MONTGOMERY_R
            );
        }

        uint256 friQueue = getPtr(ctx, MM_FRI_QUEUE);

        uint256[] memory friStepSizes = getFriStepSizes(ctx);
        uint256 nFriSteps = friStepSizes.length;
        while (friStep < nFriSteps) {
            uint256 friCosetSize = 2**friStepSizes[friStep];

            nLiveQueries = computeNextLayer(
                channelPtr,
                friQueue,
                merkleQueuePtr,
                nLiveQueries,
                friCtx,
                ctx[MM_FRI_EVAL_POINTS + friStep],
                friCosetSize
            );

            // Layer is done, verify the current layer and move to next layer.
            // ctx[mmMerkleQueue: merkleQueueIdx) holds the indices
            // and values of the merkle leaves that need verification.
            verifyMerkle(
                channelPtr,
                merkleQueuePtr,
                bytes32(ctx[MM_FRI_COMMITMENTS + friStep - 1]),
                nLiveQueries
            );

            friStep++;
        }

        verifyLastLayer(ctx, nLiveQueries);
    }
}
