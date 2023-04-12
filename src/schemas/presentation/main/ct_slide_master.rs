use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionListModify;

use crate::schemas::presentation::main::CtSlideTiming;

use crate::schemas::presentation::main::CtCommonSlideData;

use crate::schemas::presentation::main::CtHeaderFooter;

use crate::schemas::presentation::main::CtSlideLayoutIdList;

use crate::schemas::presentation::main::CtSlideTransition;

use crate::schemas::presentation::main::CtSlideMasterTextStyles;

use crate::schemas::drawing::main::CtColorMapping;

/**
 * @author : zhilong.zhou
 * @description : CT_SlideMaster
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "p:sldMaster", deserialize = "sldMaster"))]

pub struct CtSlideMaster {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@preserve")]
    pub preserve_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:p")]
    pub p_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:a")]
    pub a_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:r")]
    pub r_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:s")]
    pub s_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns")]
    pub default_namespace_attr: Option<String>,

    #[serde(rename(serialize = "p:cSld", deserialize = "cSld"))]
    pub c_sld: CtCommonSlideData,

    #[serde(rename(serialize = "p:sldLayoutIdLst", deserialize = "sldLayoutIdLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sld_layout_id_lst: Option<CtSlideLayoutIdList>,

    #[serde(rename(serialize = "p:transition", deserialize = "transition"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition: Option<CtSlideTransition>,

    #[serde(rename(serialize = "p:timing", deserialize = "timing"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<CtSlideTiming>,

    #[serde(rename(serialize = "p:hf", deserialize = "hf"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hf: Option<CtHeaderFooter>,

    #[serde(rename(serialize = "p:txStyles", deserialize = "txStyles"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_styles: Option<CtSlideMasterTextStyles>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionListModify>,

    #[serde(rename(serialize = "p:clrMap", deserialize = "clrMap"))]
    pub clr_map: CtColorMapping,
}
