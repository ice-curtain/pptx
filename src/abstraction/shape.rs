use crate::abstraction::paragraph::Paragraph;
use crate::abstraction::unit;
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

    pub fn is_shape_name(&self, shape_name: &str) -> bool {
        self.nv_sp_pr.c_nv_pr.name_attr == shape_name
    }


    pub fn add_new_paragraph(&mut self) -> &mut Paragraph {
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


    pub fn set_insets(&mut self, top: f64, left: f64, bottom: f64, right: f64) {
        self.set_top_inset(top);
        self.set_left_inset(left);
        self.set_bottom_inset(bottom);
        self.set_right_inset(right);
    }

    pub fn set_top_inset(&mut self, margin: f64) {
        let text_body = self.tx_body.as_mut();
        if let Some(text_body) = text_body {
            let bodyPr = &mut text_body.body_pr;
            bodyPr.t_ins_attr = Some(unit::toEMU(margin));
        }
    }
    pub fn set_left_inset(&mut self, margin: f64) {
        let text_body = self.tx_body.as_mut();
        if let Some(text_body) = text_body {
            let bodyPr = &mut text_body.body_pr;
            bodyPr.l_ins_attr = Some(unit::toEMU(margin));
        }
    }
    pub fn set_bottom_inset(&mut self, margin: f64) {
        let text_body = self.tx_body.as_mut();
        if let Some(text_body) = text_body {
            let bodyPr = &mut text_body.body_pr;
            bodyPr.b_ins_attr = Some(unit::toEMU(margin));
        }
    }
    pub fn set_right_inset(&mut self, margin: f64) {
        let text_body = self.tx_body.as_mut();
        if let Some(text_body) = text_body {
            let bodyPr = &mut text_body.body_pr;
            bodyPr.r_ins_attr = Some(unit::toEMU(margin));
        }
    }
}