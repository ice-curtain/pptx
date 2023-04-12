use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Point2D
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPoint2D {
    #[serde(rename = "@x")]
    pub x_attr: String,

    #[serde(rename = "@y")]
    pub y_attr: String,
}
