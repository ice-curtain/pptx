use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_PositiveFixedAngle
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPositiveFixedAngle {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
