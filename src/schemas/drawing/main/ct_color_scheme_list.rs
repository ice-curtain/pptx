use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtColorSchemeAndMapping;

/**
 * @author : zhilong.zhou
 * @description : CT_ColorSchemeList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtColorSchemeList {
    #[serde(rename(serialize = "a:extraClrScheme", deserialize = "extraClrScheme"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_clr_scheme: Option<Vec<CtColorSchemeAndMapping>>,
}
