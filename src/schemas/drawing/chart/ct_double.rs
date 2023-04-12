use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Double
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtDouble {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
