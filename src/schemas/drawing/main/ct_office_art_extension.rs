use serde::{Deserialize, Serialize};

use crate::schemas::standard::microsoft::drawing::{A14HiddenFill, A14HiddenLine, A14UseLocalDpi, A16ColId, A16CtCreationId, A16RowId, CtThemeFamily};

/**
 * @author : zhilong.zhou
 * @description : CT_OfficeArtExtension
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CtOfficeArtExtension {
    #[serde(rename = "@uri")]
    pub uri_attr: String,

    #[serde(rename(serialize = "thm15:themeFamily", deserialize = "themeFamily"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    theme_family: Option<CtThemeFamily>,

    #[serde(rename(serialize = "a16:creationId", deserialize = "creationId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_id: Option<A16CtCreationId>,

    #[serde(rename(serialize = "a14:useLocalDpi", deserialize = "useLocalDpi"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    use_local_dpi: Option<A14UseLocalDpi>,

    #[serde(rename(serialize = "a14:hiddenFill", deserialize = "hiddenFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    hidden_fill: Option<A14HiddenFill>,

    #[serde(rename(serialize = "a14:hiddenLine", deserialize = "hiddenLine"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    hidden_line: Option<A14HiddenLine>,

    #[serde(rename(serialize = "a16:rowId", deserialize = "rowId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    row_id: Option<A16RowId>,

    #[serde(rename(serialize = "a16:colId", deserialize = "colId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    col_id: Option<A16ColId>,

}
