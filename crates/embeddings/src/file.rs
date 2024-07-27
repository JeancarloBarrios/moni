use lopdf::Document;

use crate::error::FileError;

pub struct Content {
    content: String,
}

impl Content {
    pub fn from_path(path: &str) -> Result<Self, FileError> {
        let kind = infer::get_from_path(path)
            .map_err(FileError::IOError)?
            .ok_or(FileError::ParsingError(
                "file type not supported".to_string(),
            ))?;
        match kind.mime_type() {
            "application/pdf" => Content::parse_pdf(path),
            _ => Err(FileError::ParsingError("unsuported file".to_string())),
        }
    }

    fn parse_pdf(path: &str) -> Result<Content, FileError> {
        let documet = Document::load(path).map_err(FileError::PdfError)?;
        let pages = documet.get_pages();
        let mut texts = Vec::new();

        for (i, _) in pages.iter().enumerate() {
            let page_number = (i + 1) as u32;
            let text = documet.extract_text(&[page_number]);
            texts.push(text.unwrap_or_default());
        }

        Ok(Content {
            content: texts.join(""),
        })
    }

    fn gen_chunks(&self, generator: impl ChunkGenerator) -> Vec<String> {
        generator.generate(&self.content.clone())
    }
}

pub trait ChunkGenerator {
    fn generate(&self, content: &str) -> Vec<String>;
}

pub struct SentenseGenerator {}

impl SentenseGenerator {
    fn new() -> Self {
        SentenseGenerator {}
    }
}

impl ChunkGenerator for SentenseGenerator {
    fn generate(&self, content: &str) -> Vec<String> {
        unicode_segmentation::UnicodeSegmentation::unicode_sentences(content)
            .map(|s| s.to_string())
            .collect()
    }
}

pub struct ParagraphGenerator {}

impl ParagraphGenerator {
    fn new() -> Self {
        ParagraphGenerator {}
    }
}

impl ChunkGenerator for ParagraphGenerator {
    fn generate(&self, content: &str) -> Vec<String> {
        content.split("\n\n").map(String::from).collect::<Vec<_>>()
    }
}

#[test]
fn test_extract_text_from_pdf() {
    let path = "testdata/sample.pdf";
    let file = Content::from_path(path);
    assert!(file.is_ok());
    let file = file.unwrap();
    println!("Contests");
    let sentences = file.gen_chunks(SentenseGenerator::new());
    for sentence in sentences.iter().take(10) {
        println!(
            "Sentence:------------------------------------------------ {}",
            sentence
        );
    }
}
