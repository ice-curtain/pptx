use crate::schemas::drawing::main::CtTextParagraph;
use crate::schemas::presentation::main::CtShape;

pub type Shape = CtShape;

impl Shape {
    pub fn get_paragraphs(&mut self) -> Option<&mut Vec<CtTextParagraph>> {
        let mut result = None;
        match self.tx_body.as_mut() {
            Some(tx_body) => {
                let paragraphs = &mut tx_body.p;
                result = Some(paragraphs);
            }
            _ => {}
        }
        return result;
    }

    pub fn is_text_shape(&self) -> bool {
        self.tx_body.is_some()
    }

    pub fn get_shape_name(&self) -> String {
        self.nv_sp_pr.c_nv_pr.name_attr.clone()
    }


    pub fn add_new_paragraph(&mut self) -> &mut CtTextParagraph {
        match self.tx_body.as_mut() {
            Some(tx_body) => {
                let paragraphs = &mut tx_body.p;
                let mut new_paragraph = CtTextParagraph::default();
                paragraphs.push(new_paragraph);
                paragraphs.last_mut().unwrap()
            }
            _ => {
                panic!("unsupport operation");
            }
        }
    }
}