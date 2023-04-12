use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TextNormalAutofit
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextNormalAutofit {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@fontScale")]
    pub font_scale_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@lnSpcReduction")]
    pub ln_spc_reduction_attr: Option<String>,
}
