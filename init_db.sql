/*
 Navicat Premium Data Transfer

 Source Server         : cs-cn-east-71.teamcode.com
 Source Server Type    : MySQL
 Source Server Version : 80027
 Source Host           : cs-cn-east-71.teamcode.com:4064
 Source Schema         : all-in-gpt

 Target Server Type    : MySQL
 Target Server Version : 80027
 File Encoding         : 65001

 Date: 24/11/2023 19:43:32
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for Permissions
-- ----------------------------
DROP TABLE IF EXISTS `Permissions`;
CREATE TABLE `Permissions` (
  `PermissionID` int NOT NULL AUTO_INCREMENT,
  `PermissionKey` varchar(100) NOT NULL,
  `Description` text,
  PRIMARY KEY (`PermissionID`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- ----------------------------
-- Table structure for RolePermissions
-- ----------------------------
DROP TABLE IF EXISTS `RolePermissions`;
CREATE TABLE `RolePermissions` (
  `RoleID` int NOT NULL,
  `PermissionID` int NOT NULL,
  PRIMARY KEY (`RoleID`,`PermissionID`),
  KEY `PermissionID` (`PermissionID`),
  CONSTRAINT `RolePermissions_ibfk_1` FOREIGN KEY (`RoleID`) REFERENCES `Roles` (`RoleID`),
  CONSTRAINT `RolePermissions_ibfk_2` FOREIGN KEY (`PermissionID`) REFERENCES `Permissions` (`PermissionID`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- ----------------------------
-- Table structure for Roles
-- ----------------------------
DROP TABLE IF EXISTS `Roles`;
CREATE TABLE `Roles` (
  `RoleID` int NOT NULL AUTO_INCREMENT,
  `RoleName` varchar(50) NOT NULL,
  `Description` text,
  PRIMARY KEY (`RoleID`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- ----------------------------
-- Table structure for UserRoles
-- ----------------------------
DROP TABLE IF EXISTS `UserRoles`;
CREATE TABLE `UserRoles` (
  `UserID` int NOT NULL,
  `RoleID` int NOT NULL,
  PRIMARY KEY (`UserID`,`RoleID`),
  KEY `RoleID` (`RoleID`),
  CONSTRAINT `UserRoles_ibfk_1` FOREIGN KEY (`UserID`) REFERENCES `Users` (`UserID`),
  CONSTRAINT `UserRoles_ibfk_2` FOREIGN KEY (`RoleID`) REFERENCES `Roles` (`RoleID`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- ----------------------------
-- Table structure for Users
-- ----------------------------
DROP TABLE IF EXISTS `Users`;
CREATE TABLE `Users` (
  `UserID` int NOT NULL AUTO_INCREMENT,
  `Username` varchar(50) NOT NULL,
  `PasswordHash` varchar(255) NOT NULL,
  `Gender` enum('M','F') DEFAULT NULL,
  `Email` varchar(100) DEFAULT NULL,
  `IsActive` tinyint(1) NOT NULL DEFAULT '0',
  `CreatedAt` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `LastLogin` datetime DEFAULT NULL,
  PRIMARY KEY (`UserID`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

SET FOREIGN_KEY_CHECKS = 1;
