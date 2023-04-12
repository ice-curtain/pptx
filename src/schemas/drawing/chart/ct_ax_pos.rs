use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_AxPos
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAxPos {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
