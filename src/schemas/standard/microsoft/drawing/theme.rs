use crate::schemas::drawing::main::{CtLineEndProperties, CtLineJoinMiterProperties, CtOfficeArtExtensionList, CtSolidColorFillProperties};
use serde::{Deserialize, Serialize};
/**
 * xmlns:thm15=http://schemas.microsoft.com/office/thememl/2012/main
 */

#[derive(Serialize, Deserialize, Debug)]
pub struct CtThemeFamily {
    #[serde(rename = "@xmlns:thm15")]
    thm15: Option<String>,

    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@vid")]
    vid: String,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    ext_lst: Box<Option<CtOfficeArtExtensionList>>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct A16CtCreationId {
    #[serde(rename = "@xmlns:a16")]
    a16: Option<String>,


    #[serde(rename = "@id")]
    id: String,



}


#[derive(Serialize, Deserialize, Debug)]
pub struct A14UseLocalDpi {
    #[serde(rename = "@xmlns:a14")]
    a14: Option<String>,

    #[serde(rename = "@val")]
    val: String,

}
#[derive(Serialize, Deserialize, Debug)]
pub struct A16ColId {
    #[serde(rename = "@xmlns:a16")]
    a16: Option<String>,

    #[serde(rename = "@val")]
    val: String,

}


#[derive(Serialize, Deserialize, Debug)]
pub struct A16RowId {
    #[serde(rename = "@xmlns:a16")]
    a16: Option<String>,

    #[serde(rename = "@val")]
    val: String,

}


#[derive(Serialize, Deserialize, Debug)]
pub struct A14HiddenFill {
    #[serde(rename = "@xmlns:a14")]
    a14: Option<String>,



    #[serde(rename(serialize = "a:solidFill", deserialize = "solidFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    solid_fill:Option<CtSolidColorFillProperties>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct A14HiddenLine {
    #[serde(rename = "@xmlns:a14")]
    a14: Option<String>,

    #[serde(rename = "@w")]
    w: String,


    #[serde(rename(serialize = "a:solidFill", deserialize = "solidFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    solid_fill:Option<CtSolidColorFillProperties>,

    miter:Option<CtLineJoinMiterProperties>,

    #[serde(rename(serialize = "a:headEnd", deserialize = "headEnd"))]
    pub head_end: Option<CtLineEndProperties>,

    #[serde(rename(serialize = "a:tailEnd", deserialize = "tailEnd"))]
    pub tail_end: Option<CtLineEndProperties>,

}