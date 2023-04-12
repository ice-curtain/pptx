use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtBlipFillProperties;

use crate::schemas::drawing::main::CtPatternFillProperties;

use crate::schemas::drawing::main::CtSolidColorFillProperties;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtGradientFillProperties;

use crate::schemas::drawing::main::CtTextUnderlineLineFollowText;

use crate::schemas::drawing::main::CtHyperlink;

use crate::schemas::drawing::main::CtTextUnderlineFillGroupWrapper;

use crate::schemas::drawing::main::CtNoFillProperties;

use crate::schemas::drawing::main::CtColor;

use crate::schemas::drawing::main::CtTextUnderlineFillFollowText;

use crate::schemas::drawing::main::CtEffectList;

use crate::schemas::drawing::main::CtLineProperties;

use crate::schemas::drawing::main::CtTextFont;

use crate::schemas::drawing::main::CtBoolean;

use crate::schemas::drawing::main::CtGroupFillProperties;

use crate::schemas::drawing::main::CtEffectContainer;

/**
 * @author : zhilong.zhou
 * @description : CT_TextCharacterProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextCharacterProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@kumimoji")]
    pub kumimoji_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@lang")]
    pub lang_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@altLang")]
    pub alt_lang_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@sz")]
    pub sz_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@b")]
    pub b_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@i")]
    pub i_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@u")]
    pub u_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@strike")]
    pub strike_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@kern")]
    pub kern_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@cap")]
    pub cap_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@spc")]
    pub spc_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@normalizeH")]
    pub normalize_h_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@baseline")]
    pub baseline_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@noProof")]
    pub no_proof_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@dirty")]
    pub dirty_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@err")]
    pub err_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@smtClean")]
    pub smt_clean_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@smtId")]
    pub smt_id_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@bmk")]
    pub bmk_attr: Option<String>,

    #[serde(rename(serialize = "a:ln", deserialize = "ln"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ln: Option<CtLineProperties>,

    #[serde(rename(serialize = "a:highlight", deserialize = "highlight"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight: Option<CtColor>,

    #[serde(rename(serialize = "a:latin", deserialize = "latin"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latin: Option<CtTextFont>,

    #[serde(rename(serialize = "a:ea", deserialize = "ea"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ea: Option<CtTextFont>,

    #[serde(rename(serialize = "a:cs", deserialize = "cs"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cs: Option<CtTextFont>,

    #[serde(rename(serialize = "a:sym", deserialize = "sym"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sym: Option<CtTextFont>,

    #[serde(rename(serialize = "a:hlinkClick", deserialize = "hlinkClick"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hlink_click: Option<CtHyperlink>,

    #[serde(rename(serialize = "a:hlinkMouseOver", deserialize = "hlinkMouseOver"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hlink_mouse_over: Option<CtHyperlink>,

    #[serde(rename(serialize = "a:rtl", deserialize = "rtl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtl: Option<Vec<CtBoolean>>,

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

    #[serde(rename(serialize = "a:uLnTx", deserialize = "uLnTx"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_ln_tx: Option<CtTextUnderlineLineFollowText>,

    #[serde(rename(serialize = "a:uLn", deserialize = "uLn"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_ln: Option<CtLineProperties>,

    #[serde(rename(serialize = "a:uFillTx", deserialize = "uFillTx"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_fill_tx: Option<CtTextUnderlineFillFollowText>,

    #[serde(rename(serialize = "a:uFill", deserialize = "uFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_fill: Option<Box<CtTextUnderlineFillGroupWrapper>>,
}
