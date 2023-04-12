use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_AnimationChartElement
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAnimationChartElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@seriesIdx")]
    pub series_idx_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@categoryIdx")]
    pub category_idx_attr: Option<String>,

    #[serde(rename = "@bldStep")]
    pub bld_step_attr: String,
}
