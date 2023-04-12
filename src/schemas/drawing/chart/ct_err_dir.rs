use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_ErrDir
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtErrDir {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
