use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_AnimationChartBuildProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAnimationChartBuildProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@bld")]
    pub bld_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@animBg")]
    pub anim_bg_attr: Option<String>,
}
