use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
/// Common base response
pub struct BaseResponse<T> {
    pub error: bool,
    pub code: isize,
    pub result: T,
}
