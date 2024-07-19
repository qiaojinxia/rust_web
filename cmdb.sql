-- phpMyAdmin SQL Dump
-- version 5.1.1
-- https://www.phpmyadmin.net/
--
-- 主机： localhost
-- 生成日期： 2024-07-19 10:54:40
-- 服务器版本： 5.7.43-log
-- PHP 版本： 8.0.26

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- 数据库： `cmdb`
--

-- --------------------------------------------------------

--
-- 表的结构 `sys_api`
--

CREATE TABLE `sys_api` (
  `id` int(11) NOT NULL COMMENT '主键',
  `api_name` varchar(64) NOT NULL COMMENT 'Api名称',
  `api_group` varchar(64) NOT NULL COMMENT 'API组，如articles',
  `api_path` varchar(255) NOT NULL COMMENT 'API路径',
  `api_method` enum('GET','POST','PUT','DELETE') NOT NULL COMMENT '请求方法',
  `description` varchar(255) DEFAULT NULL COMMENT 'API描述',
  `create_user` varchar(64) NOT NULL COMMENT '创建者',
  `create_time` timestamp NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_user` varchar(64) DEFAULT NULL COMMENT '更新者',
  `update_time` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- --------------------------------------------------------

--
-- 表的结构 `sys_menu`
--

CREATE TABLE `sys_menu` (
  `id` int(11) NOT NULL COMMENT '主键',
  `parent_id` int(11) DEFAULT NULL COMMENT '父菜单ID',
  `type` enum('DIRECTORY','MENU') NOT NULL DEFAULT 'MENU' COMMENT '菜单项类型：目录、菜单、按钮',
  `menu_name` varchar(64) DEFAULT NULL COMMENT '菜单名称',
  `route_name` varchar(255) DEFAULT NULL COMMENT '路由名称',
  `route_path` varchar(255) DEFAULT NULL COMMENT '路由路径',
  `component` varchar(255) DEFAULT NULL COMMENT '组件路径',
  `constant` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否为常量路由',
  `i18n_key` varchar(255) DEFAULT NULL COMMENT '国际化键值',
  `roles` varchar(255) DEFAULT NULL COMMENT '角色列表',
  `keep_alive` tinyint(1) DEFAULT '0' COMMENT '是否缓存该路由',
  `icon` varchar(255) DEFAULT NULL COMMENT 'Iconify 图标',
  `local_icon` varchar(255) DEFAULT NULL COMMENT '本地图标',
  `order` int(11) DEFAULT '0' COMMENT '路由排序顺序',
  `href` varchar(255) DEFAULT NULL COMMENT '路由的外部链接',
  `hide_in_menu` tinyint(1) DEFAULT '0' COMMENT '是否在菜单中隐藏该路由',
  `active_menu` varchar(255) DEFAULT NULL COMMENT '激活的菜单键',
  `multi_tab` tinyint(1) DEFAULT '0' COMMENT '使用多个标签页',
  `fixed_index_in_tab` int(11) DEFAULT NULL COMMENT '标签页固定顺序',
  `query` json DEFAULT NULL COMMENT '路由查询参数',
  `create_user` varchar(64) NOT NULL COMMENT '创建者',
  `create_time` timestamp NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_user` varchar(64) DEFAULT NULL COMMENT '更新者',
  `update_time` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `status` tinyint(1) NOT NULL DEFAULT '1' COMMENT '菜单状态 1(enable)/2(disabled)'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- 转存表中的数据 `sys_menu`
--

INSERT INTO `sys_menu` (`id`, `parent_id`, `type`, `menu_name`, `route_name`, `route_path`, `component`, `constant`, `i18n_key`, `roles`, `keep_alive`, `icon`, `local_icon`, `order`, `href`, `hide_in_menu`, `active_menu`, `multi_tab`, `fixed_index_in_tab`, `query`, `create_user`, `create_time`, `update_user`, `update_time`, `status`) VALUES
(1, NULL, 'MENU', '首页', 'home', '/home', 'layout.base$view.home', 1, 'route.home', '', 0, 'mdi:monitor-dashboard', '', 1, NULL, 0, NULL, 0, NULL, '[]', 'admin', '2024-07-14 17:56:35', NULL, '2024-07-19 02:08:22', 1),
(2, NULL, 'MENU', '登录', 'login', '/login/:module(pwd-login|code-login|register|reset-pwd|bind-wechat)?', 'layout.blank$view.login', 1, 'route.login', '', 0, '', 'avatar', 0, NULL, 1, NULL, 0, NULL, '[]', 'admin', '2024-07-14 18:15:03', NULL, '2024-07-19 02:27:28', 1),
(3, NULL, 'DIRECTORY', '系统管理', 'manage', '/manage', 'layout.base', 0, 'route.manage', '', 0, 'carbon:cloud-service-management', NULL, 1, NULL, 0, NULL, 0, NULL, '[]', 'admin', '2024-07-14 18:15:54', NULL, '2024-07-14 18:15:54', 1),
(5, 3, 'MENU', '角色管理', 'manage_role', '/manage/role', 'view.manage_role', 0, 'route.manage_role', '', 0, 'carbon:user-role', NULL, 1, NULL, 0, NULL, 0, NULL, '[]', 'admin', '2024-07-14 18:17:06', NULL, '2024-07-14 18:19:53', 1),
(6, 3, 'MENU', '用户管理', 'manage_user', '/manage/user', 'view.manage_user', 0, 'route.manage_user', '', 0, 'ic:round-manage-accounts', NULL, 0, NULL, 0, NULL, 0, NULL, '[]', 'admin', '2024-07-14 18:17:55', NULL, '2024-07-14 18:17:56', 1),
(7, 3, 'MENU', '菜单管理', 'manage_menu', '/manage/menu', 'view.manage_menu', 0, 'route.manage_menu', '', 0, 'material-symbols:route', NULL, 2, NULL, 0, NULL, 0, NULL, '[]', 'admin', '2024-07-14 18:18:30', NULL, '2024-07-14 18:19:41', 1),
(8, 3, 'MENU', '权限管理', 'manage_permission', '/manage/permission', 'view.manage_permission', 0, 'route.manage_permission', '', 0, 'fluent:book-information-24-regular', NULL, 0, NULL, 0, NULL, 0, NULL, '[]', 'admin', '2024-07-14 18:19:32', NULL, '2024-07-14 18:19:32', 1);

-- --------------------------------------------------------

--
-- 表的结构 `sys_permission`
--

CREATE TABLE `sys_permission` (
  `id` int(11) NOT NULL COMMENT '主键',
  `permission_name` varchar(64) NOT NULL COMMENT '权限名称',
  `permission_code` varchar(64) NOT NULL COMMENT '权限Code',
  `description` varchar(255) DEFAULT NULL COMMENT '描述',
  `create_user` varchar(64) NOT NULL COMMENT '创建者',
  `create_time` timestamp NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_user` varchar(64) DEFAULT NULL COMMENT '更新者',
  `update_time` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `status` tinyint(1) NOT NULL DEFAULT '1' COMMENT '权限状态 1(enable)/2(disabled)'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- 转存表中的数据 `sys_permission`
--

INSERT INTO `sys_permission` (`id`, `permission_name`, `permission_code`, `description`, `create_user`, `create_time`, `update_user`, `update_time`, `status`) VALUES
(1, 'quanxian1', 'qx1', '权限1', 'admin', '2024-06-14 06:43:06', NULL, '2024-06-14 06:43:06', 1),
(2, 'quanxian2', 'qx2', '权限2', 'admin', '2024-06-14 06:43:25', NULL, '2024-06-14 06:43:25', 1);

-- --------------------------------------------------------

--
-- 表的结构 `sys_permission_action`
--

CREATE TABLE `sys_permission_action` (
  `id` int(11) NOT NULL COMMENT '主键',
  `permission_id` int(11) NOT NULL COMMENT '权限ID',
  `action_code` enum('CREATE','READ','UPDATE','DELETE') NOT NULL COMMENT '操作权限代码'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- 转存表中的数据 `sys_permission_action`
--

INSERT INTO `sys_permission_action` (`id`, `permission_id`, `action_code`) VALUES
(1, 1, 'CREATE'),
(2, 1, 'READ'),
(3, 2, 'DELETE'),
(4, 2, 'UPDATE');

-- --------------------------------------------------------

--
-- 表的结构 `sys_permission_target`
--

CREATE TABLE `sys_permission_target` (
  `permission_id` int(11) NOT NULL,
  `target_id` int(11) NOT NULL,
  `target_type` enum('MENU','API_GROUP') NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- 转存表中的数据 `sys_permission_target`
--

INSERT INTO `sys_permission_target` (`permission_id`, `target_id`, `target_type`) VALUES
(1, 1, 'MENU'),
(1, 3, 'MENU'),
(1, 7, 'MENU'),
(1, 9, 'MENU'),
(1, 10, 'MENU'),
(2, 2, 'MENU'),
(2, 3, 'MENU'),
(2, 5, 'MENU'),
(2, 6, 'MENU'),
(2, 8, 'MENU');

-- --------------------------------------------------------

--
-- 表的结构 `sys_role`
--

CREATE TABLE `sys_role` (
  `id` int(11) NOT NULL COMMENT '主键',
  `role_code` varchar(64) NOT NULL COMMENT '角色code',
  `role_name` varchar(64) NOT NULL COMMENT '角色名称',
  `description` varchar(255) DEFAULT NULL COMMENT '描述',
  `create_user` varchar(64) NOT NULL COMMENT '创建者',
  `create_time` timestamp NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_user` varchar(64) DEFAULT NULL COMMENT '更新者',
  `update_time` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `status` tinyint(1) NOT NULL DEFAULT '1' COMMENT '角色状态 1(enable)/2(disabled)'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- 转存表中的数据 `sys_role`
--

INSERT INTO `sys_role` (`id`, `role_code`, `role_name`, `description`, `create_user`, `create_time`, `update_user`, `update_time`, `status`) VALUES
(1, 'admin', '管理员', '系统管理员', 'admin', '2024-06-15 04:44:26', NULL, '2024-07-19 02:07:21', 1);

-- --------------------------------------------------------

--
-- 表的结构 `sys_role_permission`
--

CREATE TABLE `sys_role_permission` (
  `id` int(11) NOT NULL COMMENT '主键',
  `role_id` int(11) NOT NULL COMMENT '角色ID',
  `permission_id` int(11) NOT NULL COMMENT '权限ID',
  `create_user` varchar(64) NOT NULL COMMENT '创建者',
  `create_time` timestamp NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_user` varchar(64) DEFAULT NULL COMMENT '更新者',
  `update_time` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- 转存表中的数据 `sys_role_permission`
--

INSERT INTO `sys_role_permission` (`id`, `role_id`, `permission_id`, `create_user`, `create_time`, `update_user`, `update_time`) VALUES
(7, 1, 1, 'update_user', '2024-07-19 02:07:22', NULL, '2024-07-19 02:07:22'),
(8, 1, 2, 'update_user', '2024-07-19 02:07:23', NULL, '2024-07-19 02:07:23');

-- --------------------------------------------------------

--
-- 表的结构 `sys_user`
--

CREATE TABLE `sys_user` (
  `id` int(11) NOT NULL COMMENT '主键',
  `user_name` varchar(64) NOT NULL COMMENT '用户名',
  `nick_name` varchar(64) NOT NULL COMMENT '昵称',
  `password` varchar(256) NOT NULL COMMENT '密码',
  `email` varchar(128) NOT NULL COMMENT '邮箱',
  `gender` enum('1','2','3') NOT NULL COMMENT '性别(1.男 2.女 2.未知)',
  `mobile` varchar(15) DEFAULT NULL COMMENT '手机号码',
  `avatar` varchar(256) DEFAULT NULL COMMENT '头像',
  `create_user` varchar(64) NOT NULL COMMENT '创建者',
  `create_time` timestamp NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_user` varchar(64) DEFAULT NULL COMMENT '更新者',
  `update_time` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `last_login` timestamp NULL DEFAULT NULL COMMENT '上次登录时间',
  `status` tinyint(1) NOT NULL DEFAULT '1' COMMENT '用户状态 1(enable)/2(disabled)'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- 转存表中的数据 `sys_user`
--

INSERT INTO `sys_user` (`id`, `user_name`, `nick_name`, `password`, `email`, `gender`, `mobile`, `avatar`, `create_user`, `create_time`, `update_user`, `update_time`, `last_login`, `status`) VALUES
(1, 'Soybean', '123', '$2b$12$HxwnGWqc3cix/7oWiRp7Rus763WMjc1Aw8L/.I5yN6O6x8G1KrZN6', '123@qq.com', '1', '13222222222', NULL, 'admin', '2024-06-14 04:08:24', 'admin', '2024-06-20 08:36:07', NULL, 1);

-- --------------------------------------------------------

--
-- 表的结构 `sys_user_role`
--

CREATE TABLE `sys_user_role` (
  `id` int(11) NOT NULL COMMENT '主键',
  `user_id` int(11) NOT NULL COMMENT '用户ID',
  `role_id` int(11) NOT NULL COMMENT '角色ID',
  `create_user` varchar(64) NOT NULL COMMENT '创建者',
  `create_time` timestamp NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_user` varchar(64) DEFAULT NULL COMMENT '更新者',
  `update_time` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- 转存表中的数据 `sys_user_role`
--

INSERT INTO `sys_user_role` (`id`, `user_id`, `role_id`, `create_user`, `create_time`, `update_user`, `update_time`) VALUES
(7, 1, 1, 'admin', '2024-06-20 08:36:44', NULL, '2024-06-20 08:36:44');

--
-- 转储表的索引
--

--
-- 表的索引 `sys_api`
--
ALTER TABLE `sys_api`
  ADD PRIMARY KEY (`id`),
  ADD UNIQUE KEY `idx_method_path` (`api_path`,`api_method`),
  ADD KEY `idx_api_group` (`api_group`),
  ADD KEY `idx_api_path` (`api_path`);

--
-- 表的索引 `sys_menu`
--
ALTER TABLE `sys_menu`
  ADD PRIMARY KEY (`id`),
  ADD KEY `idx_menu_name` (`menu_name`),
  ADD KEY `idx_parent_id` (`parent_id`);

--
-- 表的索引 `sys_permission`
--
ALTER TABLE `sys_permission`
  ADD PRIMARY KEY (`id`),
  ADD UNIQUE KEY `permission_name` (`permission_name`),
  ADD UNIQUE KEY `permission_code` (`permission_code`),
  ADD KEY `idx_permission_code` (`permission_code`);

--
-- 表的索引 `sys_permission_action`
--
ALTER TABLE `sys_permission_action`
  ADD PRIMARY KEY (`id`),
  ADD KEY `idx_permission_id` (`permission_id`);

--
-- 表的索引 `sys_permission_target`
--
ALTER TABLE `sys_permission_target`
  ADD PRIMARY KEY (`permission_id`,`target_id`,`target_type`),
  ADD KEY `idx_permission_id` (`permission_id`),
  ADD KEY `idx_target_id` (`target_id`),
  ADD KEY `idx_target_type` (`target_type`);

--
-- 表的索引 `sys_role`
--
ALTER TABLE `sys_role`
  ADD PRIMARY KEY (`id`),
  ADD UNIQUE KEY `role_code` (`role_code`),
  ADD UNIQUE KEY `role_name` (`role_name`);

--
-- 表的索引 `sys_role_permission`
--
ALTER TABLE `sys_role_permission`
  ADD PRIMARY KEY (`id`),
  ADD KEY `idx_role_id` (`role_id`),
  ADD KEY `idx_permission_id` (`permission_id`);

--
-- 表的索引 `sys_user`
--
ALTER TABLE `sys_user`
  ADD PRIMARY KEY (`id`),
  ADD UNIQUE KEY `user_name` (`user_name`),
  ADD UNIQUE KEY `email` (`email`),
  ADD UNIQUE KEY `mobile` (`mobile`),
  ADD KEY `idx_user_name` (`user_name`),
  ADD KEY `idx_mobile` (`mobile`);

--
-- 表的索引 `sys_user_role`
--
ALTER TABLE `sys_user_role`
  ADD PRIMARY KEY (`id`),
  ADD KEY `idx_user_id` (`user_id`),
  ADD KEY `idx_role_id` (`role_id`);

--
-- 在导出的表使用AUTO_INCREMENT
--

--
-- 使用表AUTO_INCREMENT `sys_api`
--
ALTER TABLE `sys_api`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '主键';

--
-- 使用表AUTO_INCREMENT `sys_menu`
--
ALTER TABLE `sys_menu`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '主键', AUTO_INCREMENT=9;

--
-- 使用表AUTO_INCREMENT `sys_permission`
--
ALTER TABLE `sys_permission`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '主键', AUTO_INCREMENT=3;

--
-- 使用表AUTO_INCREMENT `sys_permission_action`
--
ALTER TABLE `sys_permission_action`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '主键', AUTO_INCREMENT=5;

--
-- 使用表AUTO_INCREMENT `sys_role`
--
ALTER TABLE `sys_role`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '主键', AUTO_INCREMENT=2;

--
-- 使用表AUTO_INCREMENT `sys_role_permission`
--
ALTER TABLE `sys_role_permission`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '主键', AUTO_INCREMENT=9;

--
-- 使用表AUTO_INCREMENT `sys_user`
--
ALTER TABLE `sys_user`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '主键', AUTO_INCREMENT=7;

--
-- 使用表AUTO_INCREMENT `sys_user_role`
--
ALTER TABLE `sys_user_role`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '主键', AUTO_INCREMENT=10;

--
-- 限制导出的表
--

--
-- 限制表 `sys_permission_action`
--
ALTER TABLE `sys_permission_action`
  ADD CONSTRAINT `sys_permission_action_ibfk_1` FOREIGN KEY (`permission_id`) REFERENCES `sys_permission` (`id`);

--
-- 限制表 `sys_permission_target`
--
ALTER TABLE `sys_permission_target`
  ADD CONSTRAINT `sys_permission_target_ibfk_1` FOREIGN KEY (`permission_id`) REFERENCES `sys_permission` (`id`) ON DELETE CASCADE;

--
-- 限制表 `sys_role_permission`
--
ALTER TABLE `sys_role_permission`
  ADD CONSTRAINT `sys_role_permission_ibfk_1` FOREIGN KEY (`role_id`) REFERENCES `sys_role` (`id`) ON DELETE CASCADE,
  ADD CONSTRAINT `sys_role_permission_ibfk_2` FOREIGN KEY (`permission_id`) REFERENCES `sys_permission` (`id`) ON DELETE CASCADE;

--
-- 限制表 `sys_user_role`
--
ALTER TABLE `sys_user_role`
  ADD CONSTRAINT `sys_user_role_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `sys_user` (`id`) ON DELETE CASCADE,
  ADD CONSTRAINT `sys_user_role_ibfk_2` FOREIGN KEY (`role_id`) REFERENCES `sys_role` (`id`) ON DELETE CASCADE;
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
