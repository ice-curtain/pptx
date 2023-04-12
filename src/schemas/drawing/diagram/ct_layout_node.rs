use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtAlgorithm;

use crate::schemas::drawing::diagram::CtConstraints;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::diagram::CtShape;

use crate::schemas::drawing::diagram::CtPresentationOf;

use crate::schemas::drawing::diagram::CtLayoutVariablePropertySet;

use crate::schemas::drawing::diagram::CtForEach;

use crate::schemas::drawing::diagram::CtRules;

use crate::schemas::drawing::diagram::CtChoose;

/**
 * @author : zhilong.zhou
 * @description : CT_LayoutNode
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtLayoutNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@name")]
    pub name_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@styleLbl")]
    pub style_lbl_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@chOrder")]
    pub ch_order_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@moveWith")]
    pub move_with_attr: Option<String>,

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

    #[serde(rename(serialize = "varLst", deserialize = "varLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub var_lst: Option<Vec<CtLayoutVariablePropertySet>>,

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
