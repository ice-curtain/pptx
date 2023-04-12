use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_AnimationDgmBuildProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAnimationDgmBuildProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@bld")]
    pub bld_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rev")]
    pub rev_attr: Option<String>,
}
