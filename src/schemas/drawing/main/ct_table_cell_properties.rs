use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtBlipFillProperties;

use crate::schemas::drawing::main::CtPatternFillProperties;

use crate::schemas::drawing::main::CtGradientFillProperties;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtCell3D;

use crate::schemas::drawing::main::CtNoFillProperties;

use crate::schemas::drawing::main::CtLineProperties;

use crate::schemas::drawing::main::CtSolidColorFillProperties;

use crate::schemas::drawing::main::CtHeaders;

use crate::schemas::drawing::main::CtGroupFillProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_TableCellProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTableCellProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@marL")]
    pub mar_l_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@marR")]
    pub mar_r_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@marT")]
    pub mar_t_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@marB")]
    pub mar_b_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@vert")]
    pub vert_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@anchor")]
    pub anchor_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@anchorCtr")]
    pub anchor_ctr_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@horzOverflow")]
    pub horz_overflow_attr: Option<String>,

    #[serde(rename(serialize = "a:lnL", deserialize = "lnL"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ln_l: Option<CtLineProperties>,

    #[serde(rename(serialize = "a:lnR", deserialize = "lnR"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ln_r: Option<CtLineProperties>,

    #[serde(rename(serialize = "a:lnT", deserialize = "lnT"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ln_t: Option<CtLineProperties>,

    #[serde(rename(serialize = "a:lnB", deserialize = "lnB"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ln_b: Option<CtLineProperties>,

    #[serde(rename(serialize = "a:lnTlToBr", deserialize = "lnTlToBr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ln_tl_to_br: Option<CtLineProperties>,

    #[serde(rename(serialize = "a:lnBlToTr", deserialize = "lnBlToTr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ln_bl_to_tr: Option<CtLineProperties>,

    #[serde(rename(serialize = "a:cell3D", deserialize = "cell3D"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell3_d: Option<CtCell3D>,

    #[serde(rename(serialize = "a:headers", deserialize = "headers"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<CtHeaders>>,

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
}
