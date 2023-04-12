use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TLIterateIntervalTime
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlIterateIntervalTime {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
