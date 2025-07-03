// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {OracleCloneLib} from "../libraries/OracleCloneLib.sol";
import {OilOracle} from "./OilOracle.sol";
import {IOilOracle} from "../interface/oracle/IOilOracle.sol";

contract OilOracleFactory is OwnableUpgradeable {
    address public implementation;
    mapping(address => address) public ownerToOracle;
    address[] public allOracles;

    event OracleCloned(address indexed owner, address indexed oracle);

    function initialize(address _implementation) public initializer {
        require(_implementation != address(0), "Implementation required");
        implementation = _implementation;
        __Ownable_init(msg.sender);
    }

    function registerOracle(
        address oilOwner,
        string memory oilCategory,
        address priceFeed,
        IOilOracle.OracleConfig memory config,
        address masterOracle
    ) external onlyOwner returns (address oracle) {
        require(oilOwner != address(0), "Owner required");
        require(ownerToOracle[oilOwner] == address(0), "Already registered");
        oracle = OracleCloneLib.createOracleClone(
            implementation,
            oilCategory,
            priceFeed,
            config,
            masterOracle
        );
        ownerToOracle[oilOwner] = oracle;
        allOracles.push(oracle);
        emit OracleCloned(oilOwner, oracle);
    }

    function getAllOracles() external view returns (address[] memory) {
        return allOracles;
    }

    function getOracle(address oilOwner) external view returns (address) {
        return ownerToOracle[oilOwner];
    }
} 