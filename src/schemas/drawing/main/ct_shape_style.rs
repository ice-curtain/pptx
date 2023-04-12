use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtFontReference;

use crate::schemas::drawing::main::CtStyleMatrixReference;

/**
 * @author : zhilong.zhou
 * @description : CT_ShapeStyle
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtShapeStyle {
    #[serde(rename(serialize = "a:lnRef", deserialize = "lnRef"))]
    pub ln_ref: CtStyleMatrixReference,

    #[serde(rename(serialize = "a:fillRef", deserialize = "fillRef"))]
    pub fill_ref: CtStyleMatrixReference,

    #[serde(rename(serialize = "a:effectRef", deserialize = "effectRef"))]
    pub effect_ref: CtStyleMatrixReference,

    #[serde(rename(serialize = "a:fontRef", deserialize = "fontRef"))]
    pub font_ref: CtFontReference,
}
