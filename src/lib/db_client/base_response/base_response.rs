use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
/// Common base response body
pub struct BaseResponseBody<T> {
    pub error: bool,
    pub code: isize,
    pub result: Vec<T>,
}
