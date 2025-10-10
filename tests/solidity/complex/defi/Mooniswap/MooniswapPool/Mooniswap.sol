// SPDX-License-Identifier: MIT

pragma solidity >=0.8.8;

import "./access/Ownable.sol";
import "./utils/ReentrancyGuard.sol";
import "./math/Math.sol";
import "./math/SafeMath.sol";
import "./ERC20/ERC20.sol";
import "./libraries/UniERC20.sol";
import "./libraries/Sqrt.sol";


interface IFactory {
    function fee() external view returns(uint256);
}


library VirtualBalance {
    using SafeMath for uint256;

    struct Data {
        uint216 balance;
        uint40 time;
    }

    uint256 public constant DECAY_PERIOD = 5 minutes;

    function set(VirtualBalance.Data storage self, uint256 balance) internal {
        self.balance = uint216(balance);
        self.time = uint40(block.timestamp);
    }

    function update(VirtualBalance.Data storage self, uint256 realBalance) internal {
        set(self, current(self, realBalance));
    }

    function scale(VirtualBalance.Data storage self, uint256 realBalance, uint256 num, uint256 denom) internal {
        set(self, current(self, realBalance).mul(num).add(denom.sub(1)).div(denom));
    }

    function current(VirtualBalance.Data memory self, uint256 realBalance) internal view returns(uint256) {
        uint256 timePassed = Math.min(DECAY_PERIOD, block.timestamp.sub(self.time));
        uint256 timeRemain = DECAY_PERIOD.sub(timePassed);
        return uint256(self.balance).mul(timeRemain).add(
            realBalance.mul(timePassed)
        ).div(DECAY_PERIOD);
    }
}


contract Mooniswap is ERC20, ReentrancyGuard, Ownable {
    using Sqrt for uint256;
    using SafeMath for uint256;
    using UniERC20 for IERC20;
    using VirtualBalance for VirtualBalance.Data;

    struct Balances {
        uint256 src;
        uint256 dst;
    }

    struct SwapVolumes {
        uint128 confirmed;
        uint128 result;
    }

    event Deposited(
        address indexed account,
        uint256 amount
    );

    event Withdrawn(
        address indexed account,
        uint256 amount
    );

    event Swapped(
        address indexed account,
        address indexed src,
        address indexed dst,
        uint256 amount,
        uint256 result,
        uint256 srcBalance,
        uint256 dstBalance,
        uint256 totalSupply,
        address referral
    );

    uint256 public constant REFERRAL_SHARE = 20; // 1/share = 5% of LPs revenue
    uint256 public constant BASE_SUPPLY = 1000;  // Total supply on first deposit
    uint256 public constant FEE_DENOMINATOR = 1e18;

    IFactory public factory;
    IERC20[] public tokens;
    mapping(IERC20 => bool) public isToken;
    mapping(IERC20 => SwapVolumes) public volumes;
    mapping(IERC20 => VirtualBalance.Data) public virtualBalancesForAddition;
    mapping(IERC20 => VirtualBalance.Data) public virtualBalancesForRemoval;

    constructor(IERC20[] memory assets, string memory name, string memory symbol) public ERC20(name, symbol) {
        require(bytes(name).length > 0, "Mooniswap: name is empty");
        require(bytes(symbol).length > 0, "Mooniswap: symbol is empty");
        require(assets.length == 2, "Mooniswap: only 2 tokens allowed");

        factory = IFactory(msg.sender);
        tokens = assets;
        for (uint i = 0; i < assets.length; i++) {
            require(!isToken[assets[i]], "Mooniswap: duplicate tokens");
            isToken[assets[i]] = true;
        }
    }

    function fee() public view returns(uint256) {
        return factory.fee();
    }

    function getTokens() external view returns(IERC20[] memory) {
        return tokens;
    }

    function decayPeriod() external pure returns(uint256) {
        return VirtualBalance.DECAY_PERIOD;
    }

    function getBalanceForAddition(IERC20 token) public view returns(uint256) {
        uint256 balance = token.uniBalanceOf(address(this));
        return Math.max(virtualBalancesForAddition[token].current(balance), balance);
    }

    function getBalanceForRemoval(IERC20 token) public view returns(uint256) {
        uint256 balance = token.uniBalanceOf(address(this));
        return Math.min(virtualBalancesForRemoval[token].current(balance), balance);
    }

    function getReturn(IERC20 src, IERC20 dst, uint256 amount) external view returns(uint256) {
        return _getReturn(src, dst, amount, getBalanceForAddition(src), getBalanceForRemoval(dst));
    }

    function deposit(uint256[] calldata amounts, uint256[] calldata minAmounts) external payable nonReentrant returns(uint256 fairSupply) {
        IERC20[] memory _tokens = tokens;
        require(amounts.length == _tokens.length, "Mooniswap: wrong amounts length");
        require(msg.value == (_tokens[0].isETH() ? amounts[0] : (_tokens[1].isETH() ? amounts[1] : 0)), "Mooniswap: wrong value usage");

        uint256[] memory realBalances = new uint256[](amounts.length);
        for (uint i = 0; i < realBalances.length; i++) {
            realBalances[i] = _tokens[i].uniBalanceOf(address(this)).sub(_tokens[i].isETH() ? msg.value : 0);
        }

        uint256 totalSupply = totalSupply();
        if (totalSupply == 0) {
            fairSupply = BASE_SUPPLY.mul(99);
            _mint(address(this), BASE_SUPPLY); // Donate up to 1%

            // Use the greatest token amount but not less than 99k for the initial supply
            for (uint i = 0; i < amounts.length; i++) {
                fairSupply = Math.max(fairSupply, amounts[i]);
            }
        }
        else {
            // Pre-compute fair supply
            fairSupply = type(uint256).max;
            for (uint i = 0; i < amounts.length; i++) {
                fairSupply = Math.min(fairSupply, totalSupply.mul(amounts[i]).div(realBalances[i]));
            }
        }

        uint256 fairSupplyCached = fairSupply;
        for (uint i = 0; i < amounts.length; i++) {
            require(amounts[i] > 0, "Mooniswap: amount is zero");
            uint256 amount = (totalSupply == 0) ? amounts[i] :
                realBalances[i].mul(fairSupplyCached).add(totalSupply - 1).div(totalSupply);
            require(amount >= minAmounts[i], "Mooniswap: minAmount not reached");

            _tokens[i].uniTransferFromSenderToThis(amount);
            if (totalSupply > 0) {
                uint256 confirmed = _tokens[i].uniBalanceOf(address(this)).sub(realBalances[i]);
                fairSupply = Math.min(fairSupply, totalSupply.mul(confirmed).div(realBalances[i]));
            }
        }

        if (totalSupply > 0) {
            for (uint i = 0; i < amounts.length; i++) {
                virtualBalancesForRemoval[_tokens[i]].scale(realBalances[i], totalSupply.add(fairSupply), totalSupply);
                virtualBalancesForAddition[_tokens[i]].scale(realBalances[i], totalSupply.add(fairSupply), totalSupply);
            }
        }

        require(fairSupply > 0, "Mooniswap: result is not enough");
        _mint(msg.sender, fairSupply);

        emit Deposited(msg.sender, fairSupply);
    }

    function withdraw(uint256 amount, uint256[] memory minReturns) external nonReentrant {
        uint256 totalSupply = totalSupply();
        _burn(msg.sender, amount);

        for (uint i = 0; i < tokens.length; i++) {
            IERC20 token = tokens[i];

            uint256 preBalance = token.uniBalanceOf(address(this));
            uint256 value = preBalance.mul(amount).div(totalSupply);
            token.uniTransfer(payable(msg.sender), value);
            require(i >= minReturns.length || value >= minReturns[i], "Mooniswap: result is not enough");

            virtualBalancesForAddition[token].scale(preBalance, totalSupply.sub(amount), totalSupply);
            virtualBalancesForRemoval[token].scale(preBalance, totalSupply.sub(amount), totalSupply);
        }

        emit Withdrawn(msg.sender, amount);
    }

    function swap(IERC20 src, IERC20 dst, uint256 amount, uint256 minReturn, address referral) external payable nonReentrant returns(uint256 result) {
        require(msg.value == (src.isETH() ? amount : 0), "Mooniswap: wrong value usage");

        Balances memory balances = Balances({
            src: src.uniBalanceOf(address(this)).sub(src.isETH() ? msg.value : 0),
            dst: dst.uniBalanceOf(address(this))
        });

        // catch possible airdrops and external balance changes for deflationary tokens
        uint256 srcAdditionBalance = Math.max(virtualBalancesForAddition[src].current(balances.src), balances.src);
        uint256 dstRemovalBalance = Math.min(virtualBalancesForRemoval[dst].current(balances.dst), balances.dst);

        src.uniTransferFromSenderToThis(amount);
        uint256 confirmed = src.uniBalanceOf(address(this)).sub(balances.src);
        return confirmed;
        result = _getReturn(src, dst, confirmed, srcAdditionBalance, dstRemovalBalance);
        require(result > 0 && result >= minReturn, "Mooniswap: return is not enough");
        dst.uniTransfer(payable(msg.sender), result);

        // Update virtual balances to the same direction only at imbalanced state
        if (srcAdditionBalance != balances.src) {
            virtualBalancesForAddition[src].set(srcAdditionBalance.add(confirmed));
        }
        if (dstRemovalBalance != balances.dst) {
            virtualBalancesForRemoval[dst].set(dstRemovalBalance.sub(result));
        }

        // Update virtual balances to the opposite direction
        virtualBalancesForRemoval[src].update(balances.src);
        virtualBalancesForAddition[dst].update(balances.dst);

        if (referral != address(0)) {
            uint256 invariantRatio = uint256(1e36);
            invariantRatio = invariantRatio.mul(balances.src.add(confirmed)).div(balances.src);
            invariantRatio = invariantRatio.mul(balances.dst.sub(result)).div(balances.dst);
            if (invariantRatio > 1e36) {
                // calculate share only if invariant increased
                uint256 referralShare = invariantRatio.sqrt().sub(1e18).mul(totalSupply()).div(1e18).div(REFERRAL_SHARE);
                if (referralShare > 0) {
                    _mint(referral, referralShare);
                }
            }
        }

        emit Swapped(msg.sender, address(src), address(dst), confirmed, result, balances.src, balances.dst, totalSupply(), referral);

        // Overflow of uint128 is desired
        volumes[src].confirmed += uint128(confirmed);
        volumes[src].result += uint128(result);
    }

    function rescueFunds(IERC20 token, uint256 amount) external nonReentrant onlyOwner {
        uint256[] memory balances = new uint256[](tokens.length);
        for (uint i = 0; i < balances.length; i++) {
            balances[i] = tokens[i].uniBalanceOf(address(this));
        }

        token.uniTransfer(payable(msg.sender), amount);

        for (uint i = 0; i < balances.length; i++) {
            require(tokens[i].uniBalanceOf(address(this)) >= balances[i], "Mooniswap: access denied");
        }
        require(balanceOf(address(this)) >= BASE_SUPPLY, "Mooniswap: access denied");
    }

    function _getReturn(IERC20 src, IERC20 dst, uint256 amount, uint256 srcBalance, uint256 dstBalance) internal view returns(uint256) {
        if (isToken[src] && isToken[dst] && src != dst && amount > 0) {
            uint256 taxedAmount = amount.sub(amount.mul(fee()).div(FEE_DENOMINATOR));
            return taxedAmount.mul(dstBalance).div(srcBalance.add(taxedAmount));
        }
    }
}
