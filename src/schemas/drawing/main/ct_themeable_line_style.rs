use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtLineProperties;

use crate::schemas::drawing::main::CtStyleMatrixReference;

/**
 * @author : zhilong.zhou
 * @description : CT_ThemeableLineStyle
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtThemeableLineStyle {
    #[serde(rename(serialize = "a:ln", deserialize = "ln"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ln: Option<CtLineProperties>,

    #[serde(rename(serialize = "a:lnRef", deserialize = "lnRef"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ln_ref: Option<CtStyleMatrixReference>,
}
