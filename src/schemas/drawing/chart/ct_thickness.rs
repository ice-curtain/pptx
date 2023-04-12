use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Thickness
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtThickness {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
