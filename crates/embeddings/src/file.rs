use lopdf::Document;

use crate::error::FileError;

type ChunkGenerator = fn(String) -> Vec<String>;

pub struct Content {
    path: String,
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
            path: path.to_string(),
            content: texts.join(""),
        })
    }

    fn gen_chunks(&self, generator: ChunkGenerator) -> Vec<String> {
        generator(self.content.clone())
    }
}

pub fn extract_sentences(content: String) -> Vec<String> {
    let g = unicode_segmentation::UnicodeSegmentation::unicode_sentences(content.as_str());
    g.map(|s| s.to_string()).collect()
}

#[test]
fn test_extract_text_from_pdf() {
    let path = "testdata/test.pdf";
    let file = Content::from_path(path);
    assert!(file.is_ok());
    let file = file.unwrap();
    println!("Contests");
    let sentences = file.gen_chunks(extract_sentences);
    for sentence in sentences.iter().take(5) {
        println!("{}", sentence);
    }
}
