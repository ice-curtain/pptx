use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtAlgorithm;

use crate::schemas::drawing::diagram::CtConstraints;

use crate::schemas::drawing::diagram::CtChoose;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::diagram::CtShape;

use crate::schemas::drawing::diagram::CtPresentationOf;

use crate::schemas::drawing::diagram::CtRules;

use crate::schemas::drawing::diagram::CtLayoutNode;

/**
 * @author : zhilong.zhou
 * @description : CT_ForEach
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtForEach {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@name")]
    pub name_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@ref")]
    pub ref_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@axis")]
    pub axis_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@ptType")]
    pub pt_type_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@hideLastTrans")]
    pub hide_last_trans_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@st")]
    pub st_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@cnt")]
    pub cnt_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@step")]
    pub step_attr: Option<String>,

    #[serde(rename(serialize = "alg", deserialize = "alg"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alg: Option<Vec<CtAlgorithm>>,

    #[serde(rename(serialize = "shape", deserialize = "shape"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<Vec<CtShape>>,

    #[serde(rename(serialize = "presOf", deserialize = "presOf"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pres_of: Option<Vec<CtPresentationOf>>,

    #[serde(rename(serialize = "constrLst", deserialize = "constrLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constr_lst: Option<Vec<CtConstraints>>,

    #[serde(rename(serialize = "ruleLst", deserialize = "ruleLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_lst: Option<Vec<CtRules>>,

    #[serde(rename(serialize = "forEach", deserialize = "forEach"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_each: Option<Vec<CtForEach>>,

    #[serde(rename(serialize = "layoutNode", deserialize = "layoutNode"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_node: Option<Vec<CtLayoutNode>>,

    #[serde(rename(serialize = "choose", deserialize = "choose"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choose: Option<Vec<CtChoose>>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<Vec<CtOfficeArtExtensionList>>,
}
