pub enum IconType {
    IconifyIcon,
    LocalIcon,
}

// 实现比较方法
impl IconType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" => Some(IconType::IconifyIcon),
            "2" => Some(IconType::LocalIcon),
            _ => None,
        }
    }
    pub fn matches(&self, s: String) -> bool {
        match self {
            IconType::IconifyIcon => s == "1",
            IconType::LocalIcon => s == "2",
        }
    }
}