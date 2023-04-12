use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtChartLines;

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtBoolean;

use crate::schemas::drawing::chart::CtDLbl;

/**
 * @author : zhilong.zhou
 * @description : CT_DLbls
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtDLbls {
    #[serde(rename(serialize = "dLbl", deserialize = "dLbl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_lbl: Option<Vec<CtDLbl>>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,

    #[serde(rename(serialize = "delete", deserialize = "delete"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<CtBoolean>,

    #[serde(rename(serialize = "showLeaderLines", deserialize = "showLeaderLines"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_leader_lines: Option<CtBoolean>,

    #[serde(rename(serialize = "leaderLines", deserialize = "leaderLines"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_lines: Option<Box<CtChartLines>>,
}
