use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtEffectStyleList;

use crate::schemas::drawing::main::CtLineStyleList;

use crate::schemas::drawing::main::CtFillStyleList;

use crate::schemas::drawing::main::CtBackgroundFillStyleList;

/**
 * @author : zhilong.zhou
 * @description : CT_StyleMatrix
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtStyleMatrix {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@name")]
    pub name_attr: Option<String>,

    #[serde(rename(serialize = "a:fillStyleLst", deserialize = "fillStyleLst"))]
    pub fill_style_lst: Box<CtFillStyleList>,

    #[serde(rename(serialize = "a:lnStyleLst", deserialize = "lnStyleLst"))]
    pub ln_style_lst: CtLineStyleList,

    #[serde(rename(serialize = "a:effectStyleLst", deserialize = "effectStyleLst"))]
    pub effect_style_lst: Box<CtEffectStyleList>,

    #[serde(rename(serialize = "a:bgFillStyleLst", deserialize = "bgFillStyleLst"))]
    pub bg_fill_style_lst: Box<CtBackgroundFillStyleList>,
}
