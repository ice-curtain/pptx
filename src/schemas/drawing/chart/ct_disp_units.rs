use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtBuiltInUnit;

use crate::schemas::drawing::chart::CtDouble;

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtDispUnitsLbl;

/**
 * @author : zhilong.zhou
 * @description : CT_DispUnits
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtDispUnits {
    #[serde(rename(serialize = "dispUnitsLbl", deserialize = "dispUnitsLbl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disp_units_lbl: Option<Box<CtDispUnitsLbl>>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,

    #[serde(rename(serialize = "custUnit", deserialize = "custUnit"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cust_unit: Option<CtDouble>,

    #[serde(rename(serialize = "builtInUnit", deserialize = "builtInUnit"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub built_in_unit: Option<CtBuiltInUnit>,
}
