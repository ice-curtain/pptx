use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Vector3D
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtVector3D {
    #[serde(rename = "@dx")]
    pub dx_attr: String,

    #[serde(rename = "@dy")]
    pub dy_attr: String,

    #[serde(rename = "@dz")]
    pub dz_attr: String,
}
