use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Skip
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSkip {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
