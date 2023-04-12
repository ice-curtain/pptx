use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtPath2DArcTo;

use crate::schemas::drawing::main::CtPath2DClose;

use crate::schemas::drawing::main::CtPath2DQuadBezierTo;

use crate::schemas::drawing::main::CtPath2DMoveTo;

use crate::schemas::drawing::main::CtPath2DCubicBezierTo;

use crate::schemas::drawing::main::CtPath2DLineTo;

/**
 * @author : zhilong.zhou
 * @description : CT_Path2D
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPath2D {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@w")]
    pub w_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@h")]
    pub h_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@fill")]
    pub fill_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@stroke")]
    pub stroke_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@extrusionOk")]
    pub extrusion_ok_attr: Option<String>,

    #[serde(rename(serialize = "a:close", deserialize = "close"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<Vec<CtPath2DClose>>,

    #[serde(rename(serialize = "a:moveTo", deserialize = "moveTo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_to: Option<Vec<CtPath2DMoveTo>>,

    #[serde(rename(serialize = "a:lnTo", deserialize = "lnTo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ln_to: Option<Vec<CtPath2DLineTo>>,

    #[serde(rename(serialize = "a:arcTo", deserialize = "arcTo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc_to: Option<Vec<CtPath2DArcTo>>,

    #[serde(rename(serialize = "a:quadBezTo", deserialize = "quadBezTo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quad_bez_to: Option<Vec<CtPath2DQuadBezierTo>>,

    #[serde(rename(serialize = "a:cubicBezTo", deserialize = "cubicBezTo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cubic_bez_to: Option<Vec<CtPath2DCubicBezierTo>>,
}
