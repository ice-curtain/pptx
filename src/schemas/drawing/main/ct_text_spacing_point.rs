use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TextSpacingPoint
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextSpacingPoint {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
