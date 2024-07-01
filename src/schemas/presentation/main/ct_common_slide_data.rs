use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtBackground;

use crate::schemas::presentation::main::CtGroupShape;

use crate::schemas::presentation::main::CtControlList;

use crate::schemas::presentation::main::CtExtensionList;

use crate::schemas::presentation::main::CtCustomerDataList;

/**
 * @author : zhilong.zhou
 * @description : CT_CommonSlideData
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCommonSlideData {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@name")]
    pub name_attr: Option<String>,

    #[serde(rename(serialize = "p:bg", deserialize = "bg"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg: Option<CtBackground>,

    #[serde(rename(serialize = "p:spTree", deserialize = "spTree"))]
    pub sp_tree: Box<CtGroupShape>,

    #[serde(rename(serialize = "p:custDataLst", deserialize = "custDataLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cust_data_lst: Option<CtCustomerDataList>,

    #[serde(rename(serialize = "p:controls", deserialize = "controls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controls: Option<CtControlList>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
