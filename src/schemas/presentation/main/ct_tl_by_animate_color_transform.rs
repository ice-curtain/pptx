use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlByHslColorTransform;

use crate::schemas::presentation::main::CtTlByRgbColorTransform;

/**
 * @author : zhilong.zhou
 * @description : CT_TLByAnimateColorTransform
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlByAnimateColorTransform {
    #[serde(rename(serialize = "p:rgb", deserialize = "rgb"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rgb: Option<CtTlByRgbColorTransform>,

    #[serde(rename(serialize = "p:hsl", deserialize = "hsl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsl: Option<CtTlByHslColorTransform>,
}
