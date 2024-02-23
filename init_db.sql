-- 用户表
CREATE TABLE sys_user (
                          id INT AUTO_INCREMENT PRIMARY KEY COMMENT '主键',
                          user_name VARCHAR(64) NOT NULL UNIQUE COMMENT '用户名',
                          password VARCHAR(256) NOT NULL COMMENT '密码', -- 密码长度增加以存储hash值
                          email VARCHAR(128) NOT NULL UNIQUE COMMENT '邮箱', -- 增加长度并添加唯一约束
                          gender ENUM('M', 'F', 'O') NOT NULL COMMENT '性别', -- 使用ENUM类型表示性别
                          mobile VARCHAR(15) UNIQUE COMMENT '手机号码', -- 手机号码长度增加
                          avatar VARCHAR(256) COMMENT '头像',
                          create_user VARCHAR(64) NOT NULL COMMENT '创建者',
                          create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                          update_user VARCHAR(64) NOT NULL COMMENT '更新者',
                          update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                          last_login TIMESTAMP COMMENT '上次登录时间',
                          is_deleted TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否删除'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- 角色表
CREATE TABLE sys_role (
                          id INT AUTO_INCREMENT PRIMARY KEY COMMENT '主键',
                          role_code VARCHAR(64) NOT NULL UNIQUE COMMENT '角色code',
                          role_name VARCHAR(64) NOT NULL UNIQUE COMMENT '角色名称',
                          description VARCHAR(255) COMMENT '描述',
                          status TINYINT(1) NOT NULL DEFAULT 1 COMMENT '角色状态 0(关)/1(开)',
                          create_user VARCHAR(64) NOT NULL COMMENT '创建者',
                          create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                          update_user VARCHAR(64) COMMENT '更新者',
                          update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


-- 权限表
CREATE TABLE sys_permission (
                                id INT AUTO_INCREMENT PRIMARY KEY COMMENT '主键',
                                permission_name VARCHAR(64)  NOT NULL UNIQUE COMMENT '权限名称',
                                description VARCHAR(255) COMMENT '描述',
                                create_user VARCHAR(64) NOT NULL COMMENT '创建者',
                                create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                                update_user VARCHAR(64) NOT NULL COMMENT '更新者',
                                update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- 菜单表
DROP TABLE IF EXISTS sys_menu;
CREATE TABLE sys_menu (
                          id INT AUTO_INCREMENT PRIMARY KEY COMMENT '主键',
                          menu_name VARCHAR(64) NOT NULL COMMENT '菜单名称',
                          permission_id INT COMMENT '关联的权限ID',
                          route VARCHAR(255) NOT NULL COMMENT '路由路径',
                          route_name VARCHAR(255) NOT NULL COMMENT '路由名称',
                          sort TINYINT DEFAULT NULL COMMENT '菜单排序',
                          parent_id INT DEFAULT NULL COMMENT '父菜单ID',
                          redirect VARCHAR(255) COMMENT '重定向地址',
                          guards TINYINT COMMENT '权限守卫',
                          type TINYINT COMMENT '菜单类型',
                          component VARCHAR(255) COMMENT '组件路径',
                          meta JSON COMMENT '元数据（包含样式、图标、附加权限、附加参数等）',
                          create_user VARCHAR(64) NOT NULL COMMENT '创建者',
                          create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                          update_user VARCHAR(64)  COMMENT '更新者',
                          update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                          status TINYINT(1) NOT NULL DEFAULT 0 COMMENT '菜单状态 0(禁用)/1(启用)',
                          is_hidden TINYINT(1) NOT NULL DEFAULT 1 COMMENT '是否隐藏',
                          is_deleted TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否删除'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


-- 用户角色关联表
CREATE TABLE sys_user_role (
                               id INT AUTO_INCREMENT PRIMARY KEY COMMENT '主键',
                               user_id INT NOT NULL COMMENT '用户ID',
                               role_id INT NOT NULL COMMENT '角色ID',
                               create_user VARCHAR(64) NOT NULL COMMENT '创建者',
                               create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                               update_user VARCHAR(64) NOT NULL COMMENT '更新者',
                               update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                               FOREIGN KEY (user_id) REFERENCES sys_user(id),
                               FOREIGN KEY (role_id) REFERENCES sys_role(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- 角色权限关联表
CREATE TABLE sys_role_permission (
                                     id INT AUTO_INCREMENT PRIMARY KEY COMMENT '主键',
                                     role_id INT NOT NULL COMMENT '角色ID',
                                     permission_id INT NOT NULL COMMENT '权限ID',
                                     create_user VARCHAR(64) NOT NULL COMMENT '创建者',
                                     create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                                     update_user VARCHAR(64) NOT NULL COMMENT '更新者',
                                     update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                                     FOREIGN KEY (role_id) REFERENCES sys_role(id),
                                     FOREIGN KEY (permission_id) REFERENCES sys_permission(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


/**
 JSON meta 字段数据 {
  "style": "菜单样式数据",
  "icon": "菜单图标数据",
  "permissions": ["附加权限1", "附加权限2"],
  "params": {"key1": "value1", "key2": "value2"}
}
**/


-- 添加外键约束，关联权限ID
ALTER TABLE sys_menu ADD CONSTRAINT FK_permission_id FOREIGN KEY (permission_id) REFERENCES sys_permission(id);

-- 添加外键约束，关联父菜单ID
ALTER TABLE sys_menu ADD CONSTRAINT FK_parent_id FOREIGN KEY (parent_id) REFERENCES sys_menu(id);


-- 用户角色关联表
ALTER TABLE sys_user_role ADD INDEX idx_user_id (user_id);
ALTER TABLE sys_user_role ADD INDEX idx_role_id (role_id);

-- 角色权限关联表
ALTER TABLE sys_role_permission ADD INDEX idx_role_id (role_id);
ALTER TABLE sys_role_permission ADD INDEX idx_permission_id (permission_id);

-- 用户表
ALTER TABLE sys_user ADD INDEX idx_user_name (user_name);
ALTER TABLE sys_user ADD INDEX idx_mobile (mobile);

-- 权限表
ALTER TABLE sys_permission ADD INDEX idx_permission_name (permission_name);


-- 菜单表
ALTER TABLE sys_menu ADD INDEX idx_menu_name (menu_name);
ALTER TABLE sys_menu ADD INDEX idx_parent_id (parent_id);
ALTER TABLE sys_menu ADD INDEX idx_permission_id (permission_id);
ALTER TABLE sys_menu ADD INDEX idx_create_user (create_user);
