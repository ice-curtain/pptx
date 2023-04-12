use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_PositivePercentage
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPositivePercentage {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
