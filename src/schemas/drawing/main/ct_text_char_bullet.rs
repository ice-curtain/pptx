use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TextCharBullet
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextCharBullet {
    #[serde(rename = "@char")]
    pub char_attr: String,
}
