use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_MarkerStyle
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtMarkerStyle {
    #[serde(rename = "@val")]
    pub val_attr: String,
}
