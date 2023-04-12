use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::diagram::CtColors;

/**
 * @author : zhilong.zhou
 * @description : CT_CTStyleLabel
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCtStyleLabel {
    #[serde(rename = "@name")]
    pub name_attr: String,

    #[serde(rename(serialize = "fillClrLst", deserialize = "fillClrLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_clr_lst: Option<CtColors>,

    #[serde(rename(serialize = "linClrLst", deserialize = "linClrLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lin_clr_lst: Option<CtColors>,

    #[serde(rename(serialize = "effectClrLst", deserialize = "effectClrLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_clr_lst: Option<CtColors>,

    #[serde(rename(serialize = "txLinClrLst", deserialize = "txLinClrLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_lin_clr_lst: Option<CtColors>,

    #[serde(rename(serialize = "txFillClrLst", deserialize = "txFillClrLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_fill_clr_lst: Option<CtColors>,

    #[serde(rename(serialize = "txEffectClrLst", deserialize = "txEffectClrLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_effect_clr_lst: Option<CtColors>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
