use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TLPoint
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlPoint {
    #[serde(rename = "@x")]
    pub x_attr: String,

    #[serde(rename = "@y")]
    pub y_attr: String,
}
