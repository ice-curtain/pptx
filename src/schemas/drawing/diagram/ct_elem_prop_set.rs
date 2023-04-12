use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtLayoutVariablePropertySet;

use crate::schemas::drawing::main::CtShapeStyle;

/**
 * @author : zhilong.zhou
 * @description : CT_ElemPropSet
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtElemPropSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@presAssocID")]
    pub pres_assoc_id_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@presName")]
    pub pres_name_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@presStyleLbl")]
    pub pres_style_lbl_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@presStyleIdx")]
    pub pres_style_idx_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@presStyleCnt")]
    pub pres_style_cnt_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@loTypeId")]
    pub lo_type_id_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@loCatId")]
    pub lo_cat_id_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@qsTypeId")]
    pub qs_type_id_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@qsCatId")]
    pub qs_cat_id_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@csTypeId")]
    pub cs_type_id_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@csCatId")]
    pub cs_cat_id_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@coherent3DOff")]
    pub coherent3_d_off_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@phldrT")]
    pub phldr_t_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@phldr")]
    pub phldr_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@custAng")]
    pub cust_ang_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@custFlipVert")]
    pub cust_flip_vert_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@custFlipHor")]
    pub cust_flip_hor_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@custSzX")]
    pub cust_sz_x_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@custSzY")]
    pub cust_sz_y_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@custScaleX")]
    pub cust_scale_x_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@custScaleY")]
    pub cust_scale_y_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@custT")]
    pub cust_t_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@custLinFactX")]
    pub cust_lin_fact_x_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@custLinFactY")]
    pub cust_lin_fact_y_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@custLinFactNeighborX")]
    pub cust_lin_fact_neighbor_x_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@custLinFactNeighborY")]
    pub cust_lin_fact_neighbor_y_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@custRadScaleRad")]
    pub cust_rad_scale_rad_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@custRadScaleInc")]
    pub cust_rad_scale_inc_attr: Option<String>,

    #[serde(rename(serialize = "presLayoutVars", deserialize = "presLayoutVars"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pres_layout_vars: Option<CtLayoutVariablePropertySet>,

    #[serde(rename(serialize = "style", deserialize = "style"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<CtShapeStyle>,
}
