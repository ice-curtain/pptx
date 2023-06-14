use crate::schemas::drawing::main::{CtRegularTextRun, CtTextCharacterProperties};

pub trait TextRun {
    ///Create a TextRun based on the paragraph's endParaRpr property
    fn new_with_end_para_rpr(end_para_rpr: &CtTextCharacterProperties) -> Self;

    /// Set the TextRun's text
    fn set_text(&mut self, value: &str);

    ///Create a TextRun based on the paragraph's endParaRpr property and text
    fn new_with_end_para_rpr_and_text(end_para_rpr: &CtTextCharacterProperties, text: &str) -> Self;
}


impl TextRun for CtRegularTextRun {
    fn new_with_end_para_rpr(end_para_rpr: &CtTextCharacterProperties) -> Self {
        CtRegularTextRun {
            r_pr: serde_json::from_str(&serde_json::to_string(end_para_rpr).unwrap()).unwrap(),
            t: String::default(),
        }
    }

    fn set_text(&mut self, value: &str) {
        self.t = String::from(value);
    }

    fn new_with_end_para_rpr_and_text(end_para_rpr: &CtTextCharacterProperties, text: &str) -> Self {
        CtRegularTextRun {
            r_pr: serde_json::from_str(&serde_json::to_string(end_para_rpr).unwrap()).unwrap(),
            t: String::from(text),
        }
    }
}