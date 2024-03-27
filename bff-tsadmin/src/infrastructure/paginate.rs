use serde::Deserialize;

pub const DEFAULT_PAGE_SIZE: u32 = 30;

#[derive(Debug, Deserialize)]
pub struct PaginateRequest {
    pub page: u32,
    pub page_size: u32,
}

impl PaginateRequest {
    pub fn page(&self) -> i32 {
        self.page as i32
    }
    pub fn page_size(&self) -> i32 {
        self.page_size as i32
    }
    pub fn offset(&self) -> i32 {
        self.page() * self.page_size()
    }
}

impl std::default::Default for PaginateRequest {
    fn default() -> Self {
        Self {
            page: 0,
            page_size: DEFAULT_PAGE_SIZE,
        }
    }
}

// impl From<pb::PaginateRequest> for PaginateRequest {
//     fn from(pr: pb::PaginateRequest) -> Self {
//         Self {
//             page: pr.page.unwrap_or(0),
//             page_size: pr.page_size.unwrap_or(DEFAULT_PAGE_SIZE),
//         }
//     }
// }

// impl Into<pb::PaginateRequest> for PaginateRequest {
//     fn into(self) -> pb::PaginateRequest {
//         pb::PaginateRequest {
//             page: Some(self.page),
//             page_size: Some(self.page_size),
//         }
//     }
// }

pub struct Paginate<T> {
    pub page: u32,
    pub page_size: u32,
    pub total: u32,
    pub total_page: u32,
    pub data: Vec<T>,
}

impl<T> Paginate<T> {
    pub fn new(page: u32, page_size: u32, total: u32, data: Vec<T>) -> Self {
        let total_page = f64::ceil(total as f64 / page_size as f64) as u32;
        Self {
            page,
            page_size,
            total,
            data,
            total_page,
        }
    }

    pub fn from_req(r: &PaginateRequest, total: u32, data: Vec<T>) -> Self {
        Self::new(r.page, r.page_size, total, data)
    }

    pub fn quick(r: &PaginateRequest, count: &(i64,), data: Vec<T>) -> Self {
        Self::from_req(r, count.0 as u32, data)
    }

    pub fn page_size(&self) -> i32 {
        self.page_size as i32
    }

    pub fn page(&self) -> i32 {
        self.page as i32
    }

    pub fn offset(&self) -> i32 {
        self.page() * self.page_size()
    }
}
