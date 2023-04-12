use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtFillProperties;

use crate::schemas::drawing::main::CtEffectProperties;

use crate::schemas::drawing::main::CtStyleMatrixReference;

/**
 * @author : zhilong.zhou
 * @description : CT_TableBackgroundStyle
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTableBackgroundStyle {
    #[serde(rename(serialize = "a:fill", deserialize = "fill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<Box<CtFillProperties>>,

    #[serde(rename(serialize = "a:fillRef", deserialize = "fillRef"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_ref: Option<CtStyleMatrixReference>,

    #[serde(rename(serialize = "a:effect", deserialize = "effect"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<Box<CtEffectProperties>>,

    #[serde(rename(serialize = "a:effectRef", deserialize = "effectRef"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_ref: Option<CtStyleMatrixReference>,
}
