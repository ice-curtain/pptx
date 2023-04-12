use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TextSpacingPercent
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextSpacingPercent {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
