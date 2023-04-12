use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtThemeableLineStyle;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_TableCellBorderStyle
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTableCellBorderStyle {
    #[serde(rename(serialize = "a:left", deserialize = "left"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<CtThemeableLineStyle>,

    #[serde(rename(serialize = "a:right", deserialize = "right"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<CtThemeableLineStyle>,

    #[serde(rename(serialize = "a:top", deserialize = "top"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<CtThemeableLineStyle>,

    #[serde(rename(serialize = "a:bottom", deserialize = "bottom"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<CtThemeableLineStyle>,

    #[serde(rename(serialize = "a:insideH", deserialize = "insideH"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inside_h: Option<CtThemeableLineStyle>,

    #[serde(rename(serialize = "a:insideV", deserialize = "insideV"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inside_v: Option<CtThemeableLineStyle>,

    #[serde(rename(serialize = "a:tl2br", deserialize = "tl2br"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tl2br: Option<CtThemeableLineStyle>,

    #[serde(rename(serialize = "a:tr2bl", deserialize = "tr2bl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr2bl: Option<CtThemeableLineStyle>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
