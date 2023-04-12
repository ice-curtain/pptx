use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_IndexRange
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtIndexRange {
    #[serde(rename = "@st")]
    pub st_attr: String,

    #[serde(rename = "@end")]
    pub end_attr: String,
}
