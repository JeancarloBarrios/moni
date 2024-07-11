use lopdf::Document;

use crate::error::FileError;

pub struct File {
    path: String,
    content: String,
}

impl File {
    pub fn parse(path: &str) -> Result<Self, FileError> {
        let kind = infer::get_from_path(path)
            .map_err(FileError::IOError)?
            .ok_or(FileError::ParsingError(
                "file type not supported".to_string(),
            ))?;
        match kind.mime_type() {
            "application/pdf" => File::parse_pdf(path),
            _ => Err(FileError::ParsingError("unsuported file".to_string())),
        }
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }

    fn parse_pdf(path: &str) -> Result<File, FileError> {
        let documet = Document::load(path).map_err(FileError::PdfError)?;
        let pages = documet.get_pages();
        let mut texts = Vec::new();

        for (i, _) in pages.iter().enumerate() {
            let page_number = (i + 1) as u32;
            let text = documet.extract_text(&[page_number]);
            texts.push(text.unwrap_or_default());
        }

        Ok(File {
            path: path.to_string(),
            content: texts.join(""),
        })
    }
}

impl GetConlent for File {
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
    let file = File::parse(path);
    assert!(file.is_ok());
    let file = file.unwrap();
    println!("Contests");
    let sentences = extract_chunks(file, extract_sentences);
    for sentence in sentences.iter().take(5) {
        println!("{}", sentence);
    }
}
