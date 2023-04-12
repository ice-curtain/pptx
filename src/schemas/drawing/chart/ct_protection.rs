use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtBoolean;

/**
 * @author : zhilong.zhou
 * @description : CT_Protection
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtProtection {
    #[serde(rename(serialize = "chartObject", deserialize = "chartObject"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_object: Option<CtBoolean>,

    #[serde(rename(serialize = "data", deserialize = "data"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CtBoolean>,

    #[serde(rename(serialize = "formatting", deserialize = "formatting"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatting: Option<CtBoolean>,

    #[serde(rename(serialize = "selection", deserialize = "selection"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<CtBoolean>,

    #[serde(rename(serialize = "userInterface", deserialize = "userInterface"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_interface: Option<CtBoolean>,
}
