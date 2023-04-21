use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtPatternFillProperties;

use crate::schemas::drawing::main::CtGradientFillProperties;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtLineJoinRound;

use crate::schemas::drawing::main::CtLineJoinBevel;

use crate::schemas::drawing::main::CtNoFillProperties;

use crate::schemas::drawing::main::CtLineEndProperties;

use crate::schemas::drawing::main::CtDashStopList;

use crate::schemas::drawing::main::CtLineJoinMiterProperties;

use crate::schemas::drawing::main::CtSolidColorFillProperties;

use crate::schemas::drawing::main::CtPresetLineDashProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_LineProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtLineProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@w")]
    pub w_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@cap")]
    pub cap_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@cmpd")]
    pub cmpd_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@algn")]
    pub algn_attr: Option<String>,

    #[serde(rename(serialize = "a:noFill", deserialize = "noFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_fill: Option<CtNoFillProperties>,

    #[serde(rename(serialize = "a:solidFill", deserialize = "solidFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solid_fill: Option<CtSolidColorFillProperties>,

    #[serde(rename(serialize = "a:gradFill", deserialize = "gradFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grad_fill: Option<CtGradientFillProperties>,

    #[serde(rename(serialize = "a:pattFill", deserialize = "pattFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patt_fill: Option<CtPatternFillProperties>,


    #[serde(rename(serialize = "a:prstDash", deserialize = "prstDash"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prst_dash: Option<CtPresetLineDashProperties>,

    #[serde(rename(serialize = "a:custDash", deserialize = "custDash"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cust_dash: Option<CtDashStopList>,

    #[serde(rename(serialize = "a:round", deserialize = "round"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub round: Option<CtLineJoinRound>,

    #[serde(rename(serialize = "a:bevel", deserialize = "bevel"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bevel: Option<CtLineJoinBevel>,

    #[serde(rename(serialize = "a:miter", deserialize = "miter"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miter: Option<CtLineJoinMiterProperties>,

    #[serde(rename(serialize = "a:headEnd", deserialize = "headEnd"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_end: Option<CtLineEndProperties>,

    #[serde(rename(serialize = "a:tailEnd", deserialize = "tailEnd"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail_end: Option<CtLineEndProperties>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,






}
