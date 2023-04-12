use quick_xml::events::Event::Decl;
use quick_xml::{Error, Reader, Writer};
use std::string::FromUtf8Error;

pub fn read_xml_dcl(xml: &str) -> Result<String, Error> {
    let mut reader = Reader::from_str(xml);
    let decl = loop {
        let r = reader.read_event();
        match r {
            Ok(event) => match event {
                Decl(e) => {
                    break Ok(e);
                }
                _ => {}
            },
            Err(e) => {
                break Err(e);
            }
        }
    };
    match decl {
        Ok(decl) => {
            let mut writer = Writer::new(Vec::new());
            writer
                .write_event(Decl(decl))
                .expect("writing xml decl should succeed");
            let result = writer.into_inner();
            let result = String::from_utf8(result);
            match result {
                Ok(result) => Ok(result),
                Err(e) => Err(Error::UnexpectedEof(String::from("parse error"))),
            }
        }
        Err(e) => Err(e),
    }
}
