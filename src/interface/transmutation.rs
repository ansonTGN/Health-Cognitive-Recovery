use std::io::{Cursor, Read};
use std::path::Path;
use anyhow::{Context, Result, anyhow};
use calamine::{Reader, Xlsx, open_workbook_from_rs};
use lopdf::Document;
use xml::reader::{EventReader, XmlEvent};

pub enum SupportedFormat { PDF, DOCX, XLSX, CSV, HTML, PlainText }

impl SupportedFormat {
    pub fn from_filename(filename: &str) -> Option<Self> {
        let ext = Path::new(filename).extension().and_then(|s| s.to_str()).map(|s| s.to_lowercase())?;
        match ext.as_str() {
            "pdf" => Some(Self::PDF),
            "docx" => Some(Self::DOCX),
            "xlsx" | "xls" => Some(Self::XLSX),
            "csv" => Some(Self::CSV),
            "html" | "htm" => Some(Self::HTML),
            "txt" | "md" | "json" | "xml" => Some(Self::PlainText),
            _ => None,
        }
    }
}

pub struct DocumentTransmuter;

impl DocumentTransmuter {
    pub fn transmute(filename: &str, data: &[u8]) -> Result<String> {
        let format = SupportedFormat::from_filename(filename)
            .ok_or_else(|| anyhow!("Formato no soportado: {}", filename))?;

        match format {
            SupportedFormat::PDF => Self::parse_pdf(data),
            SupportedFormat::DOCX => Self::parse_docx(data),
            SupportedFormat::XLSX => Self::parse_xlsx(data),
            SupportedFormat::CSV => Self::parse_csv(data),
            SupportedFormat::HTML => Self::parse_html(data),
            SupportedFormat::PlainText => String::from_utf8(data.to_vec()).map_err(|e| anyhow!("Error UTF-8: {}", e)),
        }
    }

    // ... (Implementaciones de parse_pdf, parse_docx, etc. iguales a las anteriores)
    fn parse_pdf(data: &[u8]) -> Result<String> {
        let doc = Document::load_mem(data).map_err(|e| anyhow!("Fallo PDF: {}", e))?;
        let mut text = String::new();
        for page_num in doc.get_pages().keys() {
            if let Ok(content) = doc.extract_text(&[*page_num]) {
                text.push_str(&content);
                text.push_str("\n");
            }
        }
        Ok(text)
    }

    fn parse_docx(data: &[u8]) -> Result<String> {
        let cursor = Cursor::new(data);
        let mut archive = zip::ZipArchive::new(cursor).context("ZIP inválido")?;
        let mut xml_file = archive.by_name("word/document.xml").context("DOCX sin document.xml")?;
        let mut xml_content = String::new();
        xml_file.read_to_string(&mut xml_content)?;
        let parser = EventReader::from_str(&xml_content);
        let mut text = String::new();
        for e in parser { if let Ok(XmlEvent::Characters(s)) = e { text.push_str(&s); text.push(' '); } }
        Ok(text)
    }

    fn parse_xlsx(data: &[u8]) -> Result<String> {
        let cursor = Cursor::new(data);
        let mut workbook: Xlsx<_> = open_workbook_from_rs(cursor).map_err(|e| anyhow!("Error Excel: {}", e))?;
        let mut text = String::new();
        for sheet_name in workbook.sheet_names().to_vec() {
            if let Ok(range) = workbook.worksheet_range(&sheet_name) {
                for row in range.rows() {
                    let row_str: Vec<String> = row.iter().map(|c| c.to_string()).collect();
                    text.push_str(&row_str.join(" | ")); text.push('\n');
                }
            }
        }
        Ok(text)
    }

    fn parse_csv(data: &[u8]) -> Result<String> {
        let mut rdr = csv::Reader::from_reader(Cursor::new(data));
        let mut text = String::new();
        for result in rdr.records() {
            if let Ok(record) = result {
                let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
                text.push_str(&row.join(" | ")); text.push('\n');
            }
        }
        Ok(text)
    }

    fn parse_html(data: &[u8]) -> Result<String> {
        let html_string = String::from_utf8(data.to_vec()).context("HTML no UTF-8")?;
        Ok(html2text::from_read(html_string.as_bytes(), 80))
    }
}