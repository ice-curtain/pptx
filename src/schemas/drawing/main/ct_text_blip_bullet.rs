use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtBlip;

/**
 * @author : zhilong.zhou
 * @description : CT_TextBlipBullet
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextBlipBullet {
    #[serde(rename(serialize = "a:blip", deserialize = "blip"))]
    pub blip: Box<CtBlip>,
}
