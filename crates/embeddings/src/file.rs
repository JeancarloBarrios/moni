use lopdf::Document;

use crate::error::FileError;

pub struct PdfFile {
    path: String,
    content: String,
}

impl PdfFile {
    pub fn parse(path: &str) -> Result<Self, FileError> {
        let documet = Document::load(path).map_err(FileError::PdfError)?;
        let pages = documet.get_pages();
        let mut texts = Vec::new();

        for (i, _) in pages.iter().enumerate() {
            let page_number = (i + 1) as u32;
            let text = documet.extract_text(&[page_number]);
            texts.push(text.unwrap_or_default());
        }

        Ok(PdfFile {
            path: path.to_string(),
            content: texts.join(""),
        })
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }
}

impl GetConlent for PdfFile {
    fn get_content(&self) -> String {
        self.get_content()
    }
}

pub trait GetConlent {
    fn get_content(&self) -> String;
}

pub fn extract_chunks(file: impl GetConlent, extractor: fn(String) -> Vec<String>) -> Vec<String> {
    extractor(file.get_content())
}

pub fn extract_sentences(content: String) -> Vec<String> {
    let g = unicode_segmentation::UnicodeSegmentation::unicode_sentences(content.as_str());
    g.map(|s| s.to_string()).collect()
}

#[test]
fn test_extract_text_from_pdf() {
    let path = "testdata/test.pdf";
    let file = PdfFile::parse(path);
    assert!(file.is_ok());
    let file = file.unwrap();
    println!("Contests");
    let sentences = extract_chunks(file, extract_sentences);
    for sentence in sentences.iter().take(5) {
        println!("{}", sentence);
    }
}
