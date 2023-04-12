use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtSurfaceChart;

use crate::schemas::drawing::chart::CtRadarChart;

use crate::schemas::drawing::chart::CtSerAx;

use crate::schemas::drawing::chart::CtPieChart;

use crate::schemas::drawing::chart::CtDateAx;

use crate::schemas::drawing::chart::CtValAx;

use crate::schemas::drawing::chart::CtLineChart;

use crate::schemas::drawing::chart::CtStockChart;

use crate::schemas::drawing::chart::CtDoughnutChart;

use crate::schemas::drawing::chart::CtPie3DChart;

use crate::schemas::drawing::chart::CtLayout;

use crate::schemas::drawing::chart::CtSurface3DChart;

use crate::schemas::drawing::chart::CtBar3DChart;

use crate::schemas::drawing::chart::CtDTable;

use crate::schemas::drawing::chart::CtArea3DChart;

use crate::schemas::drawing::chart::CtLine3DChart;

use crate::schemas::drawing::chart::CtOfPieChart;

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtCatAx;

use crate::schemas::drawing::chart::CtBubbleChart;

use crate::schemas::drawing::chart::CtScatterChart;

use crate::schemas::drawing::chart::CtBarChart;

use crate::schemas::drawing::chart::CtAreaChart;

use crate::schemas::drawing::main::CtShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_PlotArea
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPlotArea {
    #[serde(rename(serialize = "layout", deserialize = "layout"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<CtLayout>,

    #[serde(rename(serialize = "dTable", deserialize = "dTable"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_table: Option<Box<CtDTable>>,

    #[serde(rename(serialize = "spPr", deserialize = "spPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_pr: Option<Box<CtShapeProperties>>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,

    #[serde(rename(serialize = "areaChart", deserialize = "areaChart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_chart: Option<Vec<CtAreaChart>>,

    #[serde(rename(serialize = "area3DChart", deserialize = "area3DChart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area3_d_chart: Option<Vec<CtArea3DChart>>,

    #[serde(rename(serialize = "lineChart", deserialize = "lineChart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_chart: Option<Vec<CtLineChart>>,

    #[serde(rename(serialize = "line3DChart", deserialize = "line3DChart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line3_d_chart: Option<Vec<CtLine3DChart>>,

    #[serde(rename(serialize = "stockChart", deserialize = "stockChart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock_chart: Option<Vec<CtStockChart>>,

    #[serde(rename(serialize = "radarChart", deserialize = "radarChart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_chart: Option<Vec<CtRadarChart>>,

    #[serde(rename(serialize = "scatterChart", deserialize = "scatterChart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scatter_chart: Option<Vec<CtScatterChart>>,

    #[serde(rename(serialize = "pieChart", deserialize = "pieChart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie_chart: Option<Vec<CtPieChart>>,

    #[serde(rename(serialize = "pie3DChart", deserialize = "pie3DChart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie3_d_chart: Option<Vec<CtPie3DChart>>,

    #[serde(rename(serialize = "doughnutChart", deserialize = "doughnutChart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doughnut_chart: Option<Vec<CtDoughnutChart>>,

    #[serde(rename(serialize = "barChart", deserialize = "barChart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar_chart: Option<Vec<CtBarChart>>,

    #[serde(rename(serialize = "bar3DChart", deserialize = "bar3DChart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar3_d_chart: Option<Vec<CtBar3DChart>>,

    #[serde(rename(serialize = "ofPieChart", deserialize = "ofPieChart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub of_pie_chart: Option<Vec<CtOfPieChart>>,

    #[serde(rename(serialize = "surfaceChart", deserialize = "surfaceChart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_chart: Option<Vec<CtSurfaceChart>>,

    #[serde(rename(serialize = "surface3DChart", deserialize = "surface3DChart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface3_d_chart: Option<Vec<CtSurface3DChart>>,

    #[serde(rename(serialize = "bubbleChart", deserialize = "bubbleChart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bubble_chart: Option<Vec<CtBubbleChart>>,

    #[serde(rename(serialize = "valAx", deserialize = "valAx"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val_ax: Option<Vec<CtValAx>>,

    #[serde(rename(serialize = "catAx", deserialize = "catAx"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat_ax: Option<Vec<CtCatAx>>,

    #[serde(rename(serialize = "dateAx", deserialize = "dateAx"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_ax: Option<Vec<CtDateAx>>,

    #[serde(rename(serialize = "serAx", deserialize = "serAx"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ser_ax: Option<Vec<CtSerAx>>,
}
