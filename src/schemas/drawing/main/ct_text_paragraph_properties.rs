use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtTextNoBullet;

use crate::schemas::drawing::main::CtTextCharBullet;

use crate::schemas::drawing::main::CtTextAutonumberBullet;

use crate::schemas::drawing::main::CtTextTabStopList;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtTextBulletColorFollowText;

use crate::schemas::drawing::main::CtTextBlipBullet;

use crate::schemas::drawing::main::CtTextBulletSizePoint;

use crate::schemas::drawing::main::CtTextBulletTypefaceFollowText;

use crate::schemas::drawing::main::CtColor;

use crate::schemas::drawing::main::CtTextCharacterProperties;

use crate::schemas::drawing::main::CtTextBulletSizeFollowText;

use crate::schemas::drawing::main::CtTextBulletSizePercent;

use crate::schemas::drawing::main::CtTextFont;

use crate::schemas::drawing::main::CtTextSpacing;

/**
 * @author : zhilong.zhou
 * @description : CT_TextParagraphProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextParagraphProperties {
    #[serde(rename(serialize = "a:lnSpc", deserialize = "lnSpc"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ln_spc: Option<CtTextSpacing>,

    #[serde(rename(serialize = "a:spcBef", deserialize = "spcBef"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spc_bef: Option<CtTextSpacing>,

    #[serde(rename(serialize = "a:spcAft", deserialize = "spcAft"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spc_aft: Option<CtTextSpacing>,


    #[serde(rename(serialize = "a:buClrTx", deserialize = "buClrTx"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bu_clr_tx: Option<CtTextBulletColorFollowText>,

    #[serde(rename(serialize = "a:buClr", deserialize = "buClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bu_clr: Option<CtColor>,

    #[serde(rename(serialize = "a:buSzTx", deserialize = "buSzTx"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bu_sz_tx: Option<CtTextBulletSizeFollowText>,

    #[serde(rename(serialize = "a:buSzPct", deserialize = "buSzPct"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bu_sz_pct: Option<CtTextBulletSizePercent>,

    #[serde(rename(serialize = "a:buSzPts", deserialize = "buSzPts"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bu_sz_pts: Option<CtTextBulletSizePoint>,

    #[serde(rename(serialize = "a:buFontTx", deserialize = "buFontTx"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bu_font_tx: Option<CtTextBulletTypefaceFollowText>,

    #[serde(rename(serialize = "a:buFont", deserialize = "buFont"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bu_font: Option<CtTextFont>,

    #[serde(rename(serialize = "a:buNone", deserialize = "buNone"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bu_none: Option<CtTextNoBullet>,

    #[serde(rename(serialize = "a:buAutoNum", deserialize = "buAutoNum"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bu_auto_num: Option<CtTextAutonumberBullet>,

    #[serde(rename(serialize = "a:buChar", deserialize = "buChar"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bu_char: Option<CtTextCharBullet>,

    #[serde(rename(serialize = "a:buBlip", deserialize = "buBlip"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bu_blip: Option<Box<CtTextBlipBullet>>,


    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@marL")]
    pub mar_l_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@marR")]
    pub mar_r_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@lvl")]
    pub lvl_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@indent")]
    pub indent_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@algn")]
    pub algn_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@defTabSz")]
    pub def_tab_sz_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rtl")]
    pub rtl_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@eaLnBrk")]
    pub ea_ln_brk_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@fontAlgn")]
    pub font_algn_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@latinLnBrk")]
    pub latin_ln_brk_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@hangingPunct")]
    pub hanging_punct_attr: Option<String>,


    #[serde(rename(serialize = "a:tabLst", deserialize = "tabLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_lst: Option<CtTextTabStopList>,

    #[serde(rename(serialize = "a:defRPr", deserialize = "defRPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub def_r_pr: Option<Box<CtTextCharacterProperties>>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,

}
