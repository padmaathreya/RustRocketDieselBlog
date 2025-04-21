
use rocket::serde::{Serialize};

#[derive(Serialize)]
pub struct PaginationMeta {
   pub  current_page: u32,
   pub  per_page: u32,
   pub  from: u32,
   pub  to: u32,
   pub  total_pages: u32,
   pub  total_docs: u32,
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub records: Vec<T>,
    pub meta: PaginationMeta,
}