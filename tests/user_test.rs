use rust_web::config::globals::APP_STATE;
use rust_web::services::admin::sys_user_services::get_users_with_roles;
use rust_web::{app, common, handlers};
#[actix_rt::test]
async fn test_get_menus_by_role_id() {
    app::init().await;
    let app_state = APP_STATE.get().unwrap();
    let result = get_users_with_roles(&app_state.mysql_conn.clone(), 1, 2).await;
    // 检查结果是否成功
    assert!(result.is_ok());
    let menus = result.unwrap();
    // 检查是否至少有一个菜单返回
    // 注意：这个断言可能需要根据你的具体测试数据库内容进行调整
    assert!(!menus.is_empty());
}
