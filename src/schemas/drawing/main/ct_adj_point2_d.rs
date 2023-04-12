use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_AdjPoint2D
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAdjPoint2D {
    #[serde(rename = "@x")]
    pub x_attr: String,

    #[serde(rename = "@y")]
    pub y_attr: String,
}
