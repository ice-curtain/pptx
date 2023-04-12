use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtColorMapping;

use crate::schemas::drawing::main::CtEmptyElement;

/**
 * @author : zhilong.zhou
 * @description : CT_ColorMappingOverride
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtColorMappingOverride {
    #[serde(rename(serialize = "a:masterClrMapping", deserialize = "masterClrMapping"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_clr_mapping: Option<CtEmptyElement>,

    #[serde(rename(serialize = "a:overrideClrMapping", deserialize = "overrideClrMapping"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_clr_mapping: Option<CtColorMapping>,
}
