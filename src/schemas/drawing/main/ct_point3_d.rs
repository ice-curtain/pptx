use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Point3D
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPoint3D {
    #[serde(rename = "@x")]
    pub x_attr: String,

    #[serde(rename = "@y")]
    pub y_attr: String,

    #[serde(rename = "@z")]
    pub z_attr: String,
}
