// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {PausableUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import {ReentrancyGuardUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol";
import {IPermissionManager} from "../interface/permissions/IPermissionManager.sol";

abstract contract OilOwnerPermissionManager is IPermissionManager, Initializable, OwnableUpgradeable, PausableUpgradeable, ReentrancyGuardUpgradeable {
    function initialize(string memory, address, address) public virtual initializer {}
    function grantPermission(address, bytes4, uint256) external virtual override {}
    function grantBatchPermissions(address, bytes4[] memory, uint256) external virtual override {}
    function revokePermission(address, bytes4) external virtual override {}
    function revokeBatchPermissions(address, bytes4[] memory) external virtual override {}
    function revokeAllPermissions(address) external virtual override {}
    function hasPermission(address, bytes4) external view virtual override returns (bool) { return false; }
    function getPermission(address, bytes4) external view virtual override returns (IPermissionManager.Permission memory) { revert(); }
    function getAccountPermissions(address) external view virtual override returns (IPermissionManager.Permission[] memory) { revert(); }
    function getFunctionPermissions(bytes4) external view virtual override returns (IPermissionManager.Permission[] memory) { revert(); }
    function isPermissionExpired(address, bytes4) external view virtual override returns (bool) { return false; }
    function getActivePermissionsCount(address) external view virtual override returns (uint256) { return 0; }
    function getTotalPermissionsCount() external view virtual override returns (uint256) { return 0; }
} 