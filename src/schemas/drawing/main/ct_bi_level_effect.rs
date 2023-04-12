use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_BiLevelEffect
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtBiLevelEffect {
    #[serde(rename = "@thresh")]
    pub thresh_attr: String,
}
