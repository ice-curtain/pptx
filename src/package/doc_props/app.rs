use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename = "Properties")]
pub struct App {
    #[serde(rename = "@xmlns")]
    pub xmlns: String,

    #[serde(rename = "@xmlns:vt")]
    pub vt: String,

    #[serde(rename = "TotalTime")]
    pub total_time: i32,

    #[serde(rename = "Words")]
    pub words: i32,

    #[serde(rename = "Application")]
    pub application: String,

    #[serde(rename = "PresentationFormat")]
    pub presentation_format: String,

    #[serde(rename = "Paragraphs")]
    pub paragraphs: i32,

    #[serde(rename = "Slides")]
    pub slides: i32,

    #[serde(rename = "Notes")]
    pub notes: i32,

    #[serde(rename = "HiddenSlides")]
    pub hidden_slides: i32,

    #[serde(rename = "MMClips")]
    pub m_m_clips: i32,

    #[serde(rename = "ScaleCrop")]
    pub scale_crop: bool,

    #[serde(rename = "HeadingPairs")]
    pub heading_pairs: HeadingPairs,

    #[serde(rename = "TitlesOfParts")]
    pub titles_of_parts: TitlesOfParts,

    #[serde(rename = "Company")]
    pub company: Option<String>,

    #[serde(rename = "LinksUpToDate")]
    pub links_up_to_date: bool,



    #[serde(rename = "SharedDoc")]
    pub shared_doc: bool,

    #[serde(rename = "HyperlinksChanged")]
    pub hyperlinks_changed: bool,

    #[serde(rename = "AppVersion")]
    pub app_version: String,
}

#[derive(Serialize, Deserialize)]
pub struct HeadingPairs {
    #[serde(rename(deserialize = "vector", serialize = "vt:vector"))]
    pub vt_vector: VtVector,
}

#[derive(Serialize, Deserialize)]
pub struct VtVector {
    #[serde(rename = "@size")]
    pub size: i32,

    #[serde(rename = "@baseType")]
    pub base_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "variant", serialize = "vt:variant"))]
    pub variants: Option<Vec<VtVariant>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "lpstr", serialize = "vt:lpstr"))]
    pub lpstrs: Option<Vec<String>>,

}

#[derive(Serialize, Deserialize)]
pub struct VtVariant {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "lpstr", serialize = "vt:lpstr"))]
    pub lpstr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "i4", serialize = "vt:i4"))]
    pub i4: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TitlesOfParts {
    #[serde(rename(deserialize = "vector", serialize = "vt:vector"))]
    pub vt_vector: VtVector,
}

