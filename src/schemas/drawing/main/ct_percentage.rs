use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Percentage
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPercentage {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
