use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TLIterateIntervalPercentage
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlIterateIntervalPercentage {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
