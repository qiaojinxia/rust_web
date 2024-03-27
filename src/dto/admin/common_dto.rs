use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PaginationResponseDto<T> {
    current: u64,
    size: u64,
    total: u64,
    records: Vec<T>,
}

impl<T> PaginationResponseDto<T> {
    pub fn new(current: u64, size: u64, total: u64, records: Vec<T>) -> Self {
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
    pub current: Option<u64>,
    pub size: Option<u64>,
}