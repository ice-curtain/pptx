use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtNotesMasterIdListEntry;

/**
 * @author : zhilong.zhou
 * @description : CT_NotesMasterIdList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtNotesMasterIdList {
    #[serde(rename(serialize = "p:notesMasterId", deserialize = "notesMasterId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes_master_id: Option<CtNotesMasterIdListEntry>,
}
