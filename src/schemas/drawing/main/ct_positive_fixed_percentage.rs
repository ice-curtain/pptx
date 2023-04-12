use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_PositiveFixedPercentage
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPositiveFixedPercentage {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
