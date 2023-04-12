use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_AnimationDgmElement
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAnimationDgmElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@id")]
    pub id_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@bldStep")]
    pub bld_step_attr: Option<String>,
}
