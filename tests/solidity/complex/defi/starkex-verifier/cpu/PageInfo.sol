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

contract PageInfo {
    uint256 public constant PAGE_INFO_SIZE = 3;
    // PAGE_INFO_SIZE_IN_BYTES cannot reference PAGE_INFO_SIZE as only direct constants are
    // supported in assembly.
    uint256 public constant PAGE_INFO_SIZE_IN_BYTES = 3 * 32;

    uint256 public constant PAGE_INFO_ADDRESS_OFFSET = 0;
    uint256 public constant PAGE_INFO_SIZE_OFFSET = 1;
    uint256 public constant PAGE_INFO_HASH_OFFSET = 2;

    // A regular page entry is a (address, value) pair stored as 2 uint256 words.
    uint256 internal constant MEMORY_PAIR_SIZE = 2;
}
