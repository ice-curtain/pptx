use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtBoolean;

use crate::schemas::drawing::chart::CtDispBlanksAs;

use crate::schemas::drawing::chart::CtPivotFmts;

use crate::schemas::drawing::chart::CtPlotArea;

use crate::schemas::drawing::chart::CtSurface;

use crate::schemas::drawing::chart::CtLegend;

use crate::schemas::drawing::chart::CtTitle;

use crate::schemas::drawing::chart::CtView3D;

/**
 * @author : zhilong.zhou
 * @description : CT_Chart
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtChart {
    #[serde(rename(serialize = "title", deserialize = "title"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<CtTitle>>,

    #[serde(rename(serialize = "autoTitleDeleted", deserialize = "autoTitleDeleted"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_title_deleted: Option<CtBoolean>,

    #[serde(rename(serialize = "pivotFmts", deserialize = "pivotFmts"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_fmts: Option<Box<CtPivotFmts>>,

    #[serde(rename(serialize = "view3D", deserialize = "view3D"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view3_d: Option<CtView3D>,

    #[serde(rename(serialize = "floor", deserialize = "floor"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<Box<CtSurface>>,

    #[serde(rename(serialize = "sideWall", deserialize = "sideWall"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_wall: Option<Box<CtSurface>>,

    #[serde(rename(serialize = "backWall", deserialize = "backWall"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_wall: Option<Box<CtSurface>>,

    #[serde(rename(serialize = "plotArea", deserialize = "plotArea"))]
    pub plot_area: Box<CtPlotArea>,

    #[serde(rename(serialize = "legend", deserialize = "legend"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<Box<CtLegend>>,

    #[serde(rename(serialize = "plotVisOnly", deserialize = "plotVisOnly"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plot_vis_only: Option<CtBoolean>,

    #[serde(rename(serialize = "dispBlanksAs", deserialize = "dispBlanksAs"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disp_blanks_as: Option<CtDispBlanksAs>,

    #[serde(rename(serialize = "showDLblsOverMax", deserialize = "showDLblsOverMax"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_d_lbls_over_max: Option<CtBoolean>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}
