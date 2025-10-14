// SPDX-License-Identifier: MIT

pragma solidity >=0.6.0;

contract OtherContract {
    function externalFunction(uint256 value) external pure returns(uint256) {
        return value * 2;
    }
}

contract Test {
    enum TestEnum { Option1, Option2, Option3 }
    
    struct TestStruct {
        uint256 id;
        string name;
        TestEnum option;
    }
    
    mapping(address => uint256) public balances;
    mapping(address => TestStruct) public structs;
    
    uint256[] public numbers;
    
    event TestEvent(uint256 indexed id, address indexed sender, uint256 value);
    
    modifier onlyEven(uint256 value) {
        require(value % 2 == 0, "Value must be even");
        _;
    }

    constructor() payable public {
        balances[msg.sender] = msg.value;
    }

    function addBalance(uint256 value) public payable onlyEven(value) {
        balances[msg.sender] += value;
        emit TestEvent(numbers.length, msg.sender, value);
    }
    
    function updateStruct(uint256 id, string memory name, TestEnum option) public {
        structs[msg.sender] = TestStruct(id, name, option);
    }
    
    function recursiveFunction(uint256 value) public pure returns(uint256) {
        if (value == 0) {
            return 1;
        } else {
            return value * recursiveFunction(value - 1);
        }
    }
    
    function viewFunction() public view returns(uint256) {
        return balances[msg.sender];
    }
    
    function pureFunction(uint256 value) public pure returns(uint256) {
        return value * 2;
    }
    
    function testExternalCall(OtherContract other, uint256 value) public returns(uint256[] memory return_data) {
        try other.externalFunction(value) returns (uint256 result) {
            numbers.push(result);
        } catch {
            numbers.push(0);
        }
        return_data = numbers;
    }
    
    fallback() external payable {
        balances[msg.sender] += msg.value;
    }
    
    receive() external payable {
        balances[msg.sender] += msg.value;
    }
}
