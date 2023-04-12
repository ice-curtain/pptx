use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_AxisUnit
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAxisUnit {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
