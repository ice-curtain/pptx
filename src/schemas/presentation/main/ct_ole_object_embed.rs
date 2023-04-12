use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_OleObjectEmbed
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtOleObjectEmbed {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@followColorScheme")]
    pub follow_color_scheme_attr: Option<String>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
