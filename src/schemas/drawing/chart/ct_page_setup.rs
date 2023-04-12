use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_PageSetup
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPageSetup {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@paperSize")]
    pub paper_size_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@paperHeight")]
    pub paper_height_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@paperWidth")]
    pub paper_width_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@firstPageNumber")]
    pub first_page_number_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@orientation")]
    pub orientation_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@blackAndWhite")]
    pub black_and_white_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@draft")]
    pub draft_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@useFirstPageNumber")]
    pub use_first_page_number_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@horizontalDpi")]
    pub horizontal_dpi_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@verticalDpi")]
    pub vertical_dpi_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@copies")]
    pub copies_attr: Option<String>,
}
