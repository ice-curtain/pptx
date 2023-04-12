use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_AlphaBiLevelEffect
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAlphaBiLevelEffect {
    #[serde(rename = "@thresh")]
    pub thresh_attr: String,
}
