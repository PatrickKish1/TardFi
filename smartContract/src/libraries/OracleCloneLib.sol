// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {Clones} from "@openzeppelin/contracts/proxy/Clones.sol";
import {IOilOracle} from "../interface/oracle/IOilOracle.sol";
import {OilOwnerPermissionManager} from "../permission/OilOwnerPermissionManager.sol";

library OracleCloneLib {
    using Clones for address;

    function createOracleClone(
        address implementation,
        string memory oilCategory,
        address priceFeedAddress,
        IOilOracle.OracleConfig memory config,
        address masterOracle
    ) internal returns (address cloneAddress) {
        cloneAddress = implementation.clone();
        IOilOracle(cloneAddress).initialize(
            oilCategory,
            priceFeedAddress,
            config,
            masterOracle
        );
        return cloneAddress;
    }

    function createPermissionManagerClone(
        address implementation,
        string memory companyName,
        address masterOracle,
        address companyOwner
    ) internal returns (address cloneAddress) {
        cloneAddress = implementation.clone();
        OilOwnerPermissionManager(cloneAddress).initialize(
            companyName,
            masterOracle,
            companyOwner
        );
        return cloneAddress;
    }

    function predictCloneAddress(
        address implementation,
        bytes32 salt
    ) internal view returns (address predictedAddress) {
        return implementation.predictDeterministicAddress(salt, address(this));
    }

    function createDeterministicOracleClone(
        address implementation,
        bytes32 salt,
        string memory oilCategory,
        address priceFeedAddress,
        IOilOracle.OracleConfig memory config,
        address masterOracle
    ) internal returns (address cloneAddress) {
        cloneAddress = implementation.cloneDeterministic(salt);
        IOilOracle(cloneAddress).initialize(
            oilCategory,
            priceFeedAddress,
            config,
            masterOracle
        );
        return cloneAddress;
    }

    function validateOracleConfig(
        IOilOracle.OracleConfig memory config
    ) internal pure {
        require(config.updateInterval > 0, "Update interval must be greater than 0");
        require(config.deviationThreshold > 0, "Deviation threshold must be greater than 0");
        require(config.heartbeat > 0, "Heartbeat must be greater than 0");
        require(config.minAnswer < config.maxAnswer, "Min answer must be less than max answer");
    }

    function isValidPriceUpdate(
        uint256 currentPrice,
        uint256 newPrice,
        IOilOracle.OracleConfig memory config
    ) internal pure returns (bool isValid) {
        if (newPrice < config.minAnswer || newPrice > config.maxAnswer) {
            return false;
        }
        if (currentPrice > 0) {
            uint256 deviation = newPrice > currentPrice
                ? ((newPrice - currentPrice) * 10000) / currentPrice
                : ((currentPrice - newPrice) * 10000) / currentPrice;
            if (deviation > config.deviationThreshold) {
                return false;
            }
        }
        return true;
    }
} 