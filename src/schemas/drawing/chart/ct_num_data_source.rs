use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtNumRef;

use crate::schemas::drawing::chart::CtNumData;

/**
 * @author : zhilong.zhou
 * @description : CT_NumDataSource
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtNumDataSource {
    #[serde(rename(serialize = "numRef", deserialize = "numRef"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_ref: Option<CtNumRef>,

    #[serde(rename(serialize = "numLit", deserialize = "numLit"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_lit: Option<CtNumData>,
}
