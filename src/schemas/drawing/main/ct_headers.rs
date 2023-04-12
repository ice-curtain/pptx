use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Headers
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtHeaders {
    #[serde(rename(serialize = "a:header", deserialize = "header"))]
    pub header: Vec<String>,
}
