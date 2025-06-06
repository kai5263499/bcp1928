use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use zip::read::ZipArchive;
use scraper::{Html, Selector};

fn main() -> zip::result::ZipResult<()> {
    let path = Path::new("your_book.epub");
    let file = File::open(&path)?;

    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        if file.name().ends_with(".html") || file.name().ends_with(".xhtml") {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            
            let document = Html::parse_document(&contents);
            let selector = Selector::parse("body").unwrap(); // Or any other element you are interested in

            for element in document.select(&selector) {
                let text = element.text().collect::<Vec<_>>();
                // Here you can convert text to XML or perform other parsing
                println!("{:?}", text); // Printing for demonstration
            }
        }
    }
    Ok(())
}
