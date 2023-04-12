use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_LblAlgn
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtLblAlgn {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
