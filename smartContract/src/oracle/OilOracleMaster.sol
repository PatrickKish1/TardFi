// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {PausableUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import {ReentrancyGuardUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol";
import {OracleCloneLib} from "../libraries/OracleCloneLib.sol";
import {IOilOracle} from "../interface/oracle/IOilOracle.sol";
import {IPermissionManager} from "../interface/permissions/IPermissionManager.sol";

contract OilOracleMaster is OwnableUpgradeable, PausableUpgradeable, ReentrancyGuardUpgradeable {
    using OracleCloneLib for address;

    struct OilCompany {
        string name;
        address oracleAddress;
        address priceFeedAddress;
        address permissionManager;
        uint256 lastUpdateTime;
        bool isActive;
    }
    mapping(string => OilCompany) private _oilCompanies;
    string[] private _companyNames;
    mapping(address => string) private _oracleToCompany;
    mapping(address => string) private _permissionManagerToCompany;

    address public oracleImplementation;
    address public permissionManager;
    address public permissionManagerImplementation;

    // Config bounds
    uint256 public minUpdateInterval = 300; // 5 minutes
    uint256 public maxUpdateInterval = 86400; // 24 hours
    uint256 public minDeviationThreshold = 100; // 1%
    uint256 public maxDeviationThreshold = 5000; // 50%

    // Function selectors for permission management
    bytes4 public constant REGISTER_OIL_COMPANY_SELECTOR = bytes4(keccak256("registerOilCompany(string,address,(uint256,uint256,uint256,uint256,uint256),address)"));
    bytes4 public constant UPDATE_ORACLE_IMPL_SELECTOR = bytes4(keccak256("updateOracleImplementation(address)"));
    bytes4 public constant UPDATE_CONFIG_SELECTOR = bytes4(keccak256("updateConfiguration(uint256,uint256,uint256,uint256)"));

    event OracleImplementationUpdated(address indexed oldImpl, address indexed newImpl);
    event PermissionManagerUpdated(address indexed oldManager, address indexed newManager);
    event OilOracleRegistered(string indexed oilCategory, address indexed oracle, address indexed priceFeed, uint256 timestamp);
    event ConfigurationUpdated(uint256 minUpdateInterval, uint256 maxUpdateInterval, uint256 minDeviationThreshold, uint256 maxDeviationThreshold);
    event OilCompanyRegistered(string indexed companyName, address indexed oracle, address indexed priceFeed, uint256 timestamp);
    event CompanyPermissionManagerCreated(string indexed companyName, address indexed permissionManager, address indexed companyOwner);

    function initialize(address _oracleImplementation, address _permissionManager) public initializer {
        require(_oracleImplementation != address(0), "Invalid implementation address");
        require(_permissionManager != address(0), "Invalid permission manager address");
        oracleImplementation = _oracleImplementation;
        permissionManager = _permissionManager;
        __Ownable_init(msg.sender);
        __Pausable_init();
        __ReentrancyGuard_init();
    }

    function registerOilCompany(
        string memory companyName,
        address priceFeedAddress,
        IOilOracle.OracleConfig memory config,
        address companyOwner
    ) external whenNotPaused nonReentrant returns (address oracleAddress) {
        require(
            msg.sender == owner() ||
            IPermissionManager(permissionManager).hasPermission(msg.sender, REGISTER_OIL_COMPANY_SELECTOR),
            "Permission denied: registerOilCompany"
        );
        require(bytes(companyName).length > 0, "Company name cannot be empty");
        require(priceFeedAddress != address(0), "Price feed address cannot be zero");
        require(companyOwner != address(0), "Company owner cannot be zero address");
        require(!_oilCompanies[companyName].isActive, "Company already registered");
        _validateOracleConfig(config);
        // Create new oracle clone
        oracleAddress = OracleCloneLib.createOracleClone(
            oracleImplementation,
            companyName,
            priceFeedAddress,
            config,
            address(this)
        );
        // Create new permission manager clone
        address permissionManagerAddress = OracleCloneLib.createPermissionManagerClone(
            permissionManagerImplementation,
            companyName,
            address(this),
            companyOwner
        );
        // Register the company
        _oilCompanies[companyName] = OilCompany({
            name: companyName,
            oracleAddress: oracleAddress,
            priceFeedAddress: priceFeedAddress,
            permissionManager: permissionManagerAddress,
            lastUpdateTime: block.timestamp,
            isActive: true
        });
        _companyNames.push(companyName);
        _oracleToCompany[oracleAddress] = companyName;
        _permissionManagerToCompany[permissionManagerAddress] = companyName;
        emit OilCompanyRegistered(companyName, oracleAddress, priceFeedAddress, block.timestamp);
        emit CompanyPermissionManagerCreated(companyName, permissionManagerAddress, companyOwner);
        return oracleAddress;
    }

    function updateOracleImplementation(address newImplementation) external {
        require(
            msg.sender == owner() ||
            IPermissionManager(permissionManager).hasPermission(msg.sender, UPDATE_ORACLE_IMPL_SELECTOR),
            "Permission denied: updateOracleImplementation"
        );
        require(newImplementation != address(0), "Invalid implementation address");
        address oldImplementation = oracleImplementation;
        oracleImplementation = newImplementation;
        emit OracleImplementationUpdated(oldImplementation, newImplementation);
    }

    function updatePermissionManager(address newManager) external onlyOwner {
        require(newManager != address(0), "Invalid permission manager address");
        address oldManager = permissionManager;
        permissionManager = newManager;
        emit PermissionManagerUpdated(oldManager, newManager);
    }

    function updateConfiguration(
        uint256 _minUpdateInterval,
        uint256 _maxUpdateInterval,
        uint256 _minDeviationThreshold,
        uint256 _maxDeviationThreshold
    ) external {
        require(
            msg.sender == owner() ||
            IPermissionManager(permissionManager).hasPermission(msg.sender, UPDATE_CONFIG_SELECTOR),
            "Permission denied: updateConfiguration"
        );
        require(_minUpdateInterval < _maxUpdateInterval, "Invalid update interval range");
        require(_minDeviationThreshold < _maxDeviationThreshold, "Invalid deviation threshold range");
        minUpdateInterval = _minUpdateInterval;
        maxUpdateInterval = _maxUpdateInterval;
        minDeviationThreshold = _minDeviationThreshold;
        maxDeviationThreshold = _maxDeviationThreshold;
        emit ConfigurationUpdated(_minUpdateInterval, _maxUpdateInterval, _minDeviationThreshold, _maxDeviationThreshold);
    }

    function getOilOracle(string memory companyName) external view returns (address) {
        return _oilCompanies[companyName].oracleAddress;
    }

    function getAllOilCompanies() external view returns (string[] memory) {
        return _companyNames;
    }

    function getCompanyByOracle(address oracle) external view returns (string memory) {
        return _oracleToCompany[oracle];
    }

    function getCompanyByPermissionManager(address permissionManagerAddr) external view returns (string memory) {
        return _permissionManagerToCompany[permissionManagerAddr];
    }

    function pause() external onlyOwner {
        _pause();
    }

    function unpause() external onlyOwner {
        _unpause();
    }

    function _validateOracleConfig(IOilOracle.OracleConfig memory config) internal view {
        require(
            config.updateInterval >= minUpdateInterval &&
            config.updateInterval <= maxUpdateInterval,
            "Update interval out of bounds"
        );
        require(
            config.deviationThreshold >= minDeviationThreshold &&
            config.deviationThreshold <= maxDeviationThreshold,
            "Deviation threshold out of bounds"
        );
        require(config.heartbeat > 0, "Heartbeat must be greater than 0");
        require(config.minAnswer < config.maxAnswer, "Min answer must be less than max answer");
    }
} 