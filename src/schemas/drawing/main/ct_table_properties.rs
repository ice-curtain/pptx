use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtBlipFillProperties;

use crate::schemas::drawing::main::CtPatternFillProperties;

use crate::schemas::drawing::main::CtGradientFillProperties;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtTableStyle;

use crate::schemas::drawing::main::CtNoFillProperties;

use crate::schemas::drawing::main::CtEffectList;

use crate::schemas::drawing::main::CtSolidColorFillProperties;

use crate::schemas::drawing::main::CtGroupFillProperties;

use crate::schemas::drawing::main::CtEffectContainer;

/**
 * @author : zhilong.zhou
 * @description : CT_TableProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTableProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rtl")]
    pub rtl_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@firstRow")]
    pub first_row_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@firstCol")]
    pub first_col_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@lastRow")]
    pub last_row_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@lastCol")]
    pub last_col_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@bandRow")]
    pub band_row_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@bandCol")]
    pub band_col_attr: Option<String>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,

    #[serde(rename(serialize = "a:noFill", deserialize = "noFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_fill: Option<CtNoFillProperties>,

    #[serde(rename(serialize = "a:solidFill", deserialize = "solidFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solid_fill: Option<CtSolidColorFillProperties>,

    #[serde(rename(serialize = "a:gradFill", deserialize = "gradFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grad_fill: Option<CtGradientFillProperties>,

    #[serde(rename(serialize = "a:blipFill", deserialize = "blipFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blip_fill: Option<Box<CtBlipFillProperties>>,

    #[serde(rename(serialize = "a:pattFill", deserialize = "pattFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patt_fill: Option<CtPatternFillProperties>,

    #[serde(rename(serialize = "a:grpFill", deserialize = "grpFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grp_fill: Option<CtGroupFillProperties>,

    #[serde(rename(serialize = "a:effectLst", deserialize = "effectLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_lst: Option<Box<CtEffectList>>,

    #[serde(rename(serialize = "a:effectDag", deserialize = "effectDag"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_dag: Option<Box<CtEffectContainer>>,

    #[serde(rename(serialize = "a:tableStyle", deserialize = "tableStyle"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_style: Option<Box<CtTableStyle>>,

    #[serde(rename(serialize = "a:tableStyleId", deserialize = "tableStyleId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_style_id: Option<String>,
}
