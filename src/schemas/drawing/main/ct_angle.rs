use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Angle
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAngle {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
