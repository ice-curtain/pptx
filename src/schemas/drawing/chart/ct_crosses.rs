use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Crosses
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCrosses {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
