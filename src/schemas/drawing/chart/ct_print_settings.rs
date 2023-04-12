use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtPageMargins;

use crate::schemas::drawing::chart::CtHeaderFooter;

use crate::schemas::drawing::chart::CtPageSetup;

/**
 * @author : zhilong.zhou
 * @description : CT_PrintSettings
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPrintSettings {
    #[serde(rename(serialize = "headerFooter", deserialize = "headerFooter"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_footer: Option<CtHeaderFooter>,

    #[serde(rename(serialize = "pageMargins", deserialize = "pageMargins"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_margins: Option<CtPageMargins>,

    #[serde(rename(serialize = "pageSetup", deserialize = "pageSetup"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_setup: Option<CtPageSetup>,
}
