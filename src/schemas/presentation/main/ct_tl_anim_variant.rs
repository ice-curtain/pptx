use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlAnimVariantStringVal;

use crate::schemas::presentation::main::CtTlAnimVariantBooleanVal;

use crate::schemas::drawing::main::CtColor;

use crate::schemas::presentation::main::CtTlAnimVariantFloatVal;

use crate::schemas::presentation::main::CtTlAnimVariantIntegerVal;

/**
 * @author : zhilong.zhou
 * @description : CT_TLAnimVariant
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlAnimVariant {
    #[serde(rename(serialize = "p:boolVal", deserialize = "boolVal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool_val: Option<CtTlAnimVariantBooleanVal>,

    #[serde(rename(serialize = "p:intVal", deserialize = "intVal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int_val: Option<CtTlAnimVariantIntegerVal>,

    #[serde(rename(serialize = "p:fltVal", deserialize = "fltVal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flt_val: Option<CtTlAnimVariantFloatVal>,

    #[serde(rename(serialize = "p:strVal", deserialize = "strVal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str_val: Option<CtTlAnimVariantStringVal>,

    #[serde(rename(serialize = "p:clrVal", deserialize = "clrVal"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_val: Option<CtColor>,
}
