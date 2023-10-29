use std::net::SocketAddr;
use serde::{Serialize, Deserialize};
pub type Tasks = Vec<Task>;
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub created_by: SocketAddr,
    pub title: String,
}



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
