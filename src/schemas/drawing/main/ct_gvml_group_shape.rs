use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtGvmlConnector;

use crate::schemas::drawing::main::CtGvmlPicture;

use crate::schemas::drawing::main::CtGvmlGraphicalObjectFrame;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtGvmlTextShape;

use crate::schemas::drawing::main::CtGroupShapeProperties;

use crate::schemas::drawing::main::CtGvmlGroupShapeNonVisual;

use crate::schemas::drawing::main::CtGvmlShape;

/**
 * @author : zhilong.zhou
 * @description : CT_GvmlGroupShape
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "a:lockedCanvas", deserialize = "lockedCanvas"))]

pub struct CtGvmlGroupShape {
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

    #[serde(rename(serialize = "a:nvGrpSpPr", deserialize = "nvGrpSpPr"))]
    pub nv_grp_sp_pr: CtGvmlGroupShapeNonVisual,

    #[serde(rename(serialize = "a:grpSpPr", deserialize = "grpSpPr"))]
    pub grp_sp_pr: Box<CtGroupShapeProperties>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,

    #[serde(rename(serialize = "a:txSp", deserialize = "txSp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_sp: Option<Vec<CtGvmlTextShape>>,

    #[serde(rename(serialize = "a:sp", deserialize = "sp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp: Option<Vec<CtGvmlShape>>,

    #[serde(rename(serialize = "a:cxnSp", deserialize = "cxnSp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cxn_sp: Option<Vec<CtGvmlConnector>>,

    #[serde(rename(serialize = "a:pic", deserialize = "pic"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pic: Option<Vec<CtGvmlPicture>>,

    #[serde(rename(serialize = "a:graphicFrame", deserialize = "graphicFrame"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphic_frame: Option<Vec<CtGvmlGraphicalObjectFrame>>,

    #[serde(rename(serialize = "a:grpSp", deserialize = "grpSp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grp_sp: Option<Vec<CtGvmlGroupShape>>,
}
