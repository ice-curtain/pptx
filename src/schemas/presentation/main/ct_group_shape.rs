use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionListModify;

use crate::schemas::presentation::main::CtShape;

use crate::schemas::presentation::main::CtGraphicalObjectFrame;

use crate::schemas::presentation::main::CtGroupShapeNonVisual;

use crate::schemas::presentation::main::CtConnector;

use crate::schemas::presentation::main::CtRel;

use crate::schemas::presentation::main::CtPicture;

use crate::schemas::drawing::main::CtGroupShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_GroupShape
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGroupShape {
    #[serde(rename(serialize = "p:nvGrpSpPr", deserialize = "nvGrpSpPr"))]
    pub nv_grp_sp_pr: CtGroupShapeNonVisual,

    #[serde(rename(serialize = "p:grpSpPr", deserialize = "grpSpPr"))]
    pub grp_sp_pr: CtGroupShapeProperties,


    #[serde(rename(serialize = "p:sp", deserialize = "sp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp: Option<Vec<CtShape>>,

    #[serde(rename(serialize = "p:grpSp", deserialize = "grpSp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grp_sp: Option<Vec<CtGroupShape>>,

    #[serde(rename(serialize = "p:graphicFrame", deserialize = "graphicFrame"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphic_frame: Option<Vec<CtGraphicalObjectFrame>>,

    #[serde(rename(serialize = "p:cxnSp", deserialize = "cxnSp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cxn_sp: Option<Vec<CtConnector>>,

    #[serde(rename(serialize = "p:pic", deserialize = "pic"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pic: Option<Vec<CtPicture>>,

    #[serde(rename(serialize = "p:contentPart", deserialize = "contentPart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_part: Option<Vec<CtRel>>,


    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionListModify>,
}
