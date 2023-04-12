use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_GeomRect
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGeomRect {
    #[serde(rename = "@l")]
    pub l_attr: String,

    #[serde(rename = "@t")]
    pub t_attr: String,

    #[serde(rename = "@r")]
    pub r_attr: String,

    #[serde(rename = "@b")]
    pub b_attr: String,
}
