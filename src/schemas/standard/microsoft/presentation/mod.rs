use serde::{Serialize,Deserialize};
use crate::schemas::drawing::main::CtSRgbColor;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "p188:authorLst", deserialize = "authorLst"))]
pub struct CtAuthorList {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:p188")]
    pub p188_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:a")]
    pub a_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:r")]
    pub r_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "p188:author", deserialize = "author"))]
    authors: Option<Vec<CtAuthor>>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CtAuthor {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@initials")]
    initials: String,

    #[serde(rename = "@userId")]
    user_id: String,

    #[serde(rename = "@providerId")]
    provider_id: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct P14CtCreationId {
    #[serde(rename = "@xmlns:p14")]
    #[serde(skip_serializing_if = "Option::is_none")]
    p14: Option<String>,


    #[serde(rename = "@val")]
    val: String,



}
#[derive(Serialize, Deserialize, Debug)]
pub struct P14CtDiscardImageEditData {
    #[serde(rename = "@xmlns:p14")]
    #[serde(skip_serializing_if = "Option::is_none")]
    p14: Option<String>,


    #[serde(rename = "@val")]
    val: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct P14CtDefaultImageDpi {
    #[serde(rename = "@xmlns:p14")]
    #[serde(skip_serializing_if = "Option::is_none")]
    p14: Option<String>,


    #[serde(rename = "@val")]
    val: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct P15ChartTrackingRefBased {
    #[serde(rename = "@xmlns:p15")]
    #[serde(skip_serializing_if = "Option::is_none")]
    p15: Option<String>,


    #[serde(rename = "@val")]
    val: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct P15NotesGuideLst {
    #[serde(rename = "@xmlns:p15")]
    #[serde(skip_serializing_if = "Option::is_none")]
    p15: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct P15SldGuideLst {
    #[serde(rename = "@xmlns:p15")]
    #[serde(skip_serializing_if = "Option::is_none")]
    p15: Option<String>,


    #[serde(rename(serialize = "p15:guide", deserialize = "guide"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    p15_guide:Option<Vec<P15SldGuide>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct P15SldGuide {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id:Option<String>,

    #[serde(rename = "@orient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    orient:Option<String>,

    #[serde(rename = "@pos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pos:Option<String>,

    #[serde(rename(serialize = "p15:clr", deserialize = "clr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    p15_clr:Option<P15Clr>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct P15Clr {
    #[serde(rename(serialize = "a:srgbClr", deserialize = "srgbClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    s_rgb_clr:Option<CtSRgbColor>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct P14ModId{
    #[serde(rename = "@xmlns:p14")]
    #[serde(skip_serializing_if = "Option::is_none")]
    p14: Option<String>,

    #[serde(rename = "@val")]
    val: String,
}