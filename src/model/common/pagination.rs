use serde::{Deserialize, Serialize};

/// Pagination result structure as described in docs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pagination<T> {
    pub total: u64,
    pub page: u64,
    pub size: u64,
    pub pages: u64,
    pub list: Vec<T>,
}

impl<T> Pagination<T> {
    pub fn new(total: u64, page: u64, size: u64, list: Vec<T>) -> Self {
        let pages = if size > 0 { (total + size - 1) / size } else { 0 };
        Pagination { total, page, size, pages, list }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pagination_basic() {
        let p = Pagination::new(100, 1, 10, vec![1,2,3]);
        assert_eq!(p.pages, 10);
    }
}
