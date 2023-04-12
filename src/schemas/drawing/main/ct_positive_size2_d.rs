use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_PositiveSize2D
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPositiveSize2D {
    #[serde(rename = "@cx")]
    pub cx_attr: String,

    #[serde(rename = "@cy")]
    pub cy_attr: String,
}
