use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_FixedPercentage
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtFixedPercentage {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
