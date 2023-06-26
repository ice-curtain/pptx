use crate::abstraction::textrun::TextRun;
use crate::schemas::drawing::main::{CtRegularTextRun, CtTextParagraph};


pub type Paragraph = CtTextParagraph;

pub trait IParagraph {
    fn get_runs(&mut self) -> Option<&mut Vec<CtRegularTextRun>>;
    fn add_new_text_run(&mut self) -> &mut CtRegularTextRun;
    fn add_new_text_run_with_text(&mut self, text: &str) -> &mut CtRegularTextRun;
    fn is_empty(&self) -> bool;
}


impl IParagraph for CtTextParagraph {
    fn get_runs(&mut self) -> Option<&mut Vec<CtRegularTextRun>> {
        self.r.as_mut()
    }

    fn add_new_text_run(&mut self) -> &mut CtRegularTextRun {
        match self.r.as_mut() {
            Some(runs) => {
                runs.push(CtRegularTextRun::new_with_end_para_rpr(self.end_para_r_pr.as_ref().unwrap()));
                runs.last_mut().unwrap()
            }
            None => { panic!("run can not empty") }
        }
    }

    fn add_new_text_run_with_text(&mut self, text: &str) -> &mut CtRegularTextRun {
        match self.r.as_mut() {
            Some(runs) => {
                runs.push(CtRegularTextRun::new_with_end_para_rpr_and_text(self.end_para_r_pr.as_ref().unwrap(), text));
                runs.last_mut().unwrap()
            }
            None => { panic!("run can not empty") }
        }
    }

    fn is_empty(&self) -> bool {
        self.r.is_none() || self.r.as_ref().unwrap().is_empty()
    }
}