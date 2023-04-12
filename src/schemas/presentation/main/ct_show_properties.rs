use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtIndexRange;

use crate::schemas::presentation::main::CtCustomShowId;

use crate::schemas::presentation::main::CtShowInfoBrowse;

use crate::schemas::presentation::main::CtEmpty;

use crate::schemas::drawing::main::CtColor;

use crate::schemas::presentation::main::CtShowInfoKiosk;

use crate::schemas::presentation::main::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_ShowProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtShowProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@loop")]
    pub loop_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@showNarration")]
    pub show_narration_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@showAnimation")]
    pub show_animation_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@useTimings")]
    pub use_timings_attr: Option<String>,

    #[serde(rename(serialize = "p:penClr", deserialize = "penClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pen_clr: Option<CtColor>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,

    #[serde(rename(serialize = "p:present", deserialize = "present"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub present: Option<CtEmpty>,

    #[serde(rename(serialize = "p:browse", deserialize = "browse"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browse: Option<CtShowInfoBrowse>,

    #[serde(rename(serialize = "p:kiosk", deserialize = "kiosk"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiosk: Option<CtShowInfoKiosk>,

    #[serde(rename(serialize = "p:sldAll", deserialize = "sldAll"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sld_all: Option<CtEmpty>,

    #[serde(rename(serialize = "p:sldRg", deserialize = "sldRg"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sld_rg: Option<CtIndexRange>,

    #[serde(rename(serialize = "p:custShow", deserialize = "custShow"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cust_show: Option<CtCustomShowId>,
}
