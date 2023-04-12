use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtBoolean;

/**
 * @author : zhilong.zhou
 * @description : CT_ExternalData
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtExternalData {
    #[serde(rename = "@id")]
    pub id_attr: String,

    #[serde(rename(serialize = "autoUpdate", deserialize = "autoUpdate"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_update: Option<CtBoolean>,
}
