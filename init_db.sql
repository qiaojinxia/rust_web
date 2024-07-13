-- 用户表
DROP TABLE IF EXISTS sys_user;
CREATE TABLE sys_user (
                          id INT AUTO_INCREMENT PRIMARY KEY COMMENT '主键',
                          user_name VARCHAR(64) NOT NULL UNIQUE COMMENT '用户名',
                          nick_name VARCHAR(64) NOT NULL  COMMENT '昵称',
                          password VARCHAR(256) NOT NULL COMMENT '密码',
                          email VARCHAR(128) NOT NULL UNIQUE COMMENT '邮箱',
                          gender ENUM('1', '2', '3') NOT NULL COMMENT '性别(1.男 2.女 2.未知)',
                          mobile VARCHAR(15) UNIQUE COMMENT '手机号码',
                          avatar VARCHAR(256) COMMENT '头像',
                          create_user VARCHAR(64) NOT NULL COMMENT '创建者',
                          create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                          update_user VARCHAR(64)  COMMENT '更新者',
                          update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                          last_login TIMESTAMP COMMENT '上次登录时间',
                          status TINYINT(1) NOT NULL DEFAULT 1 COMMENT '用户状态 1(enable)/2(disabled)',
                          INDEX idx_user_name (user_name),
                          INDEX idx_mobile (mobile)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- 角色表
DROP TABLE IF EXISTS sys_role;
CREATE TABLE sys_role (
                          id INT AUTO_INCREMENT PRIMARY KEY COMMENT '主键',
                          role_code VARCHAR(64) NOT NULL UNIQUE COMMENT '角色code',
                          role_name VARCHAR(64) NOT NULL UNIQUE COMMENT '角色名称',
                          description VARCHAR(255) COMMENT '描述',
                          create_user VARCHAR(64) NOT NULL COMMENT '创建者',
                          create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                          update_user VARCHAR(64) COMMENT '更新者',
                          update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                          status TINYINT(1) NOT NULL DEFAULT 1 COMMENT '角色状态 1(enable)/2(disabled)'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- 删除 sys_permission 表如果存在
DROP TABLE IF EXISTS sys_permission;
-- 权限表
DROP TABLE IF EXISTS sys_permission;
    CREATE TABLE sys_permission (
                          id INT AUTO_INCREMENT PRIMARY KEY COMMENT '主键',
                          permission_name VARCHAR(64) NOT NULL UNIQUE  COMMENT '权限名称',
                          permission_code VARCHAR(64)  NOT NULL UNIQUE COMMENT '权限Code',
                          description VARCHAR(255) COMMENT '描述',
                          create_user VARCHAR(64) NOT NULL COMMENT '创建者',
                          create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                          update_user VARCHAR(64) COMMENT '更新者',
                          update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                          status TINYINT(1) NOT NULL DEFAULT 1 COMMENT '权限状态 1(enable)/2(disabled)',
                          INDEX idx_permission_code (permission_code)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


-- 删除 sys_menu 表如果存在
DROP TABLE IF EXISTS sys_menu;
-- 创建 sys_menu 表，并在创建时添加索引
CREATE TABLE sys_menu (
                          id INT AUTO_INCREMENT PRIMARY KEY COMMENT '主键',
                          parent_id INT DEFAULT NULL COMMENT '父菜单ID',
                          type ENUM('DIRECTORY', 'MENU', 'BUTTON') NOT NULL DEFAULT 'MENU' COMMENT '菜单项类型：目录、菜单、按钮',
                          menu_name VARCHAR(64) COMMENT '菜单名称',
                          route_name VARCHAR(255) COMMENT '路由名称',
                          route_path VARCHAR(255) COMMENT '路由路径',
                          component VARCHAR(255) COMMENT '组件路径',
                          constant TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否为常量路由',
                          meta JSON COMMENT '元数据，包含字段 icon, icon_type, i18n_key, path_param, order, keep_alive, href, active_menu, multiTab, fixed_index_in_tab, query',
                          create_user VARCHAR(64) NOT NULL COMMENT '创建者',
                          create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                          update_user VARCHAR(64) COMMENT '更新者',
                          update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                          is_hidden TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否隐藏',
                          status TINYINT(1) NOT NULL DEFAULT 1 COMMENT '菜单状态 1(enable)/2(disabled)',
                          INDEX idx_menu_name (menu_name),
                          INDEX idx_parent_id (parent_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


-- API管理表
DROP TABLE IF EXISTS sys_api;
CREATE TABLE sys_api (
                         id INT AUTO_INCREMENT PRIMARY KEY COMMENT '主键',
                         api_name VARCHAR(64) NOT NULL COMMENT 'Api名称',
                         api_group VARCHAR(64) NOT NULL COMMENT 'API组，如articles',
                         api_path VARCHAR(255) NOT NULL COMMENT 'API路径',
                         api_method ENUM('GET', 'POST', 'PUT', 'DELETE') NOT NULL COMMENT '请求方法',
                         description VARCHAR(255) COMMENT 'API描述',
                         create_user VARCHAR(64) NOT NULL COMMENT '创建者',
                         create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                         update_user VARCHAR(64) COMMENT '更新者',
                         update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                         INDEX idx_api_group (api_group),
                         INDEX idx_api_path (api_path),
                         UNIQUE INDEX idx_method_path (api_path, api_method)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


-- 权限操作表
DROP TABLE IF EXISTS sys_permission_action;
CREATE TABLE sys_permission_action (
                          id INT AUTO_INCREMENT PRIMARY KEY COMMENT '主键',
                          permission_id INT NOT NULL COMMENT '权限ID',
                          action_code ENUM('CREATE', 'READ', 'UPDATE', 'DELETE') NOT NULL COMMENT '操作权限代码',
                          FOREIGN KEY (permission_id) REFERENCES sys_permission(id) ,
                          INDEX idx_permission_id (permission_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;



DROP TABLE IF EXISTS sys_permission_target;
CREATE TABLE sys_permission_target (
                                       permission_id INT NOT NULL,
                                       target_id INT NOT NULL,
                                       target_type ENUM('MENU', 'API_GROUP') NOT NULL,
                                       PRIMARY KEY (permission_id, target_id, target_type),
                                       FOREIGN KEY (permission_id) REFERENCES sys_permission(id) ON DELETE CASCADE,
                                       INDEX idx_permission_id (permission_id),
                                       INDEX idx_target_id (target_id),
                                       INDEX idx_target_type (target_type)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


-- 用户角色关联表
DROP TABLE IF EXISTS sys_user_role;
CREATE TABLE sys_user_role (
                          id INT AUTO_INCREMENT PRIMARY KEY COMMENT '主键',
                          user_id INT NOT NULL COMMENT '用户ID',
                          role_id INT NOT NULL COMMENT '角色ID',
                          create_user VARCHAR(64) NOT NULL COMMENT '创建者',
                          create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                          update_user VARCHAR(64) COMMENT '更新者',
                          update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                          FOREIGN KEY (user_id) REFERENCES sys_user(id) ON DELETE CASCADE,
                          FOREIGN KEY (role_id) REFERENCES sys_role(id) ON DELETE CASCADE,
                          INDEX idx_user_id (user_id),
                          INDEX idx_role_id (role_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;



-- 角色权限关联表
DROP TABLE IF EXISTS sys_role_permission;
CREATE TABLE sys_role_permission (
                          id INT AUTO_INCREMENT PRIMARY KEY COMMENT '主键',
                          role_id INT NOT NULL COMMENT '角色ID',
                          permission_id INT NOT NULL COMMENT '权限ID',
                          create_user VARCHAR(64) NOT NULL COMMENT '创建者',
                          create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                          update_user VARCHAR(64) COMMENT '更新者',
                          update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                          FOREIGN KEY (role_id) REFERENCES sys_role(id) ON DELETE CASCADE,
                          FOREIGN KEY (permission_id) REFERENCES sys_permission(id) ON DELETE CASCADE,
                          INDEX idx_role_id (role_id),
                          INDEX idx_permission_id (permission_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


/**
 JSON meta 字段数据 {
  "style": "菜单样式数据",
  "icon": "菜单图标数据",
  "permissions": ["附加权限1", "附加权限2"],
  "params": {"key1": "value1", "key2": "value2"}
}
**/

