use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtTablePartStyle;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtTableBackgroundStyle;

/**
 * @author : zhilong.zhou
 * @description : CT_TableStyle
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTableStyle {
    #[serde(rename = "@styleId")]
    pub style_id_attr: String,

    #[serde(rename = "@styleName")]
    pub style_name_attr: String,

    #[serde(rename(serialize = "a:tblBg", deserialize = "tblBg"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbl_bg: Option<Box<CtTableBackgroundStyle>>,

    #[serde(rename(serialize = "a:wholeTbl", deserialize = "wholeTbl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whole_tbl: Option<Box<CtTablePartStyle>>,

    #[serde(rename(serialize = "a:band1H", deserialize = "band1H"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band1_h: Option<Box<CtTablePartStyle>>,

    #[serde(rename(serialize = "a:band2H", deserialize = "band2H"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band2_h: Option<Box<CtTablePartStyle>>,

    #[serde(rename(serialize = "a:band1V", deserialize = "band1V"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band1_v: Option<Box<CtTablePartStyle>>,

    #[serde(rename(serialize = "a:band2V", deserialize = "band2V"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band2_v: Option<Box<CtTablePartStyle>>,

    #[serde(rename(serialize = "a:lastCol", deserialize = "lastCol"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_col: Option<Box<CtTablePartStyle>>,

    #[serde(rename(serialize = "a:firstCol", deserialize = "firstCol"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_col: Option<Box<CtTablePartStyle>>,

    #[serde(rename(serialize = "a:lastRow", deserialize = "lastRow"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_row: Option<Box<CtTablePartStyle>>,

    #[serde(rename(serialize = "a:seCell", deserialize = "seCell"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se_cell: Option<Box<CtTablePartStyle>>,

    #[serde(rename(serialize = "a:swCell", deserialize = "swCell"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sw_cell: Option<Box<CtTablePartStyle>>,

    #[serde(rename(serialize = "a:firstRow", deserialize = "firstRow"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_row: Option<Box<CtTablePartStyle>>,

    #[serde(rename(serialize = "a:neCell", deserialize = "neCell"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ne_cell: Option<Box<CtTablePartStyle>>,

    #[serde(rename(serialize = "a:nwCell", deserialize = "nwCell"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nw_cell: Option<Box<CtTablePartStyle>>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}
