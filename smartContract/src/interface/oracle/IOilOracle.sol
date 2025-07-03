// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

interface IOilOracle {
    struct OracleConfig {
        uint256 updateInterval;
        uint256 deviationThreshold;
        uint256 heartbeat;
        uint256 minAnswer;
        uint256 maxAnswer;
    }

    struct PriceData {
        uint256 price;
        uint256 timestamp;
        uint256 roundId;
        bool isValid;
    }

    event PriceUpdated(uint256 indexed roundId, uint256 price, uint256 timestamp, string oilCategory);
    event ConfigUpdated(uint256 updateInterval, uint256 deviationThreshold, uint256 heartbeat, uint256 timestamp);

    function initialize(
        string memory oilCategory,
        address priceFeedAddress,
        OracleConfig memory config,
        address masterOracle
    ) external;
} 