use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PaginationResponseDto<T> {
    current: u32,
    size: u32,
    total: u64,
    records: Vec<T>,
}

impl<T> PaginationResponseDto<T> {
    pub fn new(current: u32, size: u32, total: u64, records: Vec<T>) -> Self {
        PaginationResponseDto {
            current,
            size,
            total,
            records,
        }
    }
}


#[derive(Deserialize, Copy, Clone)]
pub struct PaginationQueryDto {
    pub page: Option<u32>,
    pub size: Option<u32>,
}