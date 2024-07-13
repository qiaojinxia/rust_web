use std::cell::RefCell;
use std::rc::Rc;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct RouteDto {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub component: Option<String>,
    pub meta: Option<serde_json::Value>,
    pub children: Option<Vec<Rc<RefCell<RouteDto>>>>
}


#[derive(Serialize, Debug)]
pub struct RoleMenuResponseDto {
    pub home: String,
    pub routes: Vec<Rc<RefCell<RouteDto>>>,
}
