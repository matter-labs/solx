//! { "cases": [ {
//!     "name": "test",
//!     "inputs": [
//!         {
//!             "method": "f",
//!             "calldata": [
//!             ]
//!         }
//!     ],
//!     "expected": [
//!         "1", "1", "1", "1"
//!     ]
//! } ] }

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.20;

contract Test {
    function()
        external
        returns (address, address, address, address, address) ft2;
    TestA testA;

    constructor() {
        testA = new TestA();
    }

    // function f()
    //     public
    //     returns (
    //         address r1,
    //         address r2,
    //         address r3,
    //         address r4,
    //         address r5,
    //         address r6,
    //         address r7
    //     )
    // {
    //     ft2 = this.f2;

    //     try testA.fA(ft2) returns (
    //         address r1,
    //         address r2,
    //         address r3,
    //         address r4,
    //         address r5,
    //         address r6,
    //         address r7
    //     ) {
    //         return (r1, r2, r3, r4, r5, r6, r7);
    //     } catch {
    //         return (
    //             address(0),
    //             address(0),
    //             address(0),
    //             address(0),
    //             address(0),
    //             address(0),
    //             address(0)
    //         );
    //     }
    // }

    function f() public returns (bool, bool, bool, bool) {
        ft2 = this.f2;

        try testA.fA(ft2) returns (
            address r1,
            address r2,
            address r3,
            address r4,
            address r5,
            address r6,
            address r7
        ) {
            return (r1 == r5, r5 == r7, r2 == r4, r4 == r6);
        } catch {
            return (false, false, false, false);
        }
    }

    function f2() public returns (address, address, address, address, address) {
        require(msg.sender == address(testA));
        address target = address(new TestB());
        (, bytes memory data) = target.delegatecall(
            abi.encodeWithSignature("fB()")
        );
        (address r1, address r2) = abi.decode(data, (address, address));
        return (r1, r2, target, msg.sender, address(this));
    }
}

contract TestA {
    function fA(
        function()
            external
            returns (address, address, address, address, address) _f
    )
        public
        returns (address, address, address, address, address, address, address)
    {
        (address r1, address r2, address r3, address r4, address r5) = _f();
        return (r1, r2, r3, r4, r5, address(this), msg.sender);
    }
}

contract TestB {
    function fB() public returns (address, address) {
        return (address(this), msg.sender);
    }
}
