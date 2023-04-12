use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chartDrawing::CtGraphicFrame;

use crate::schemas::drawing::chartDrawing::CtShape;

use crate::schemas::drawing::chartDrawing::CtGroupShapeNonVisual;

use crate::schemas::drawing::main::CtGroupShapeProperties;

use crate::schemas::drawing::chartDrawing::CtPicture;

use crate::schemas::drawing::chartDrawing::CtConnector;

/**
 * @author : zhilong.zhou
 * @description : CT_GroupShape
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGroupShape {
    #[serde(rename(serialize = "nvGrpSpPr", deserialize = "nvGrpSpPr"))]
    pub nv_grp_sp_pr: CtGroupShapeNonVisual,

    #[serde(rename(serialize = "grpSpPr", deserialize = "grpSpPr"))]
    pub grp_sp_pr: Box<CtGroupShapeProperties>,

    #[serde(rename(serialize = "sp", deserialize = "sp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp: Option<Vec<CtShape>>,

    #[serde(rename(serialize = "grpSp", deserialize = "grpSp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grp_sp: Option<Vec<CtGroupShape>>,

    #[serde(rename(serialize = "graphicFrame", deserialize = "graphicFrame"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphic_frame: Option<Vec<CtGraphicFrame>>,

    #[serde(rename(serialize = "cxnSp", deserialize = "cxnSp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cxn_sp: Option<Vec<CtConnector>>,

    #[serde(rename(serialize = "pic", deserialize = "pic"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pic: Option<Vec<CtPicture>>,
}
