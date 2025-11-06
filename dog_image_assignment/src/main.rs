use serde::Deserialize;
use std::error::Error;
use std::fs::{File, create_dir_all};
use std::io::copy;
use std::path::Path;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum AppError {
    Http(u16),
    Json(String),
    Network(String),
    Io(std::io::Error),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Http(code) => write!(f, "HTTP error: {}", code),
            AppError::Json(e) => write!(f, "JSON parse error: {}", e),
            AppError::Network(e) => write!(f, "Network error: {}", e),
            AppError::Io(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl Error for AppError {}
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

fn fetch_random_dog_image() -> Result<DogImage, AppError> {
    let url = "https://dog.ceo/api/breeds/image/random";

    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                response
                    .into_json::<DogImage>()
                    .map_err(|e| AppError::Json(e.to_string()))
            } else {
                Err(AppError::Http(response.status()))
            }
        }
        Err(e) => Err(AppError::Network(e.to_string())),
    }
}

fn download_image(url: &str, index: usize) -> Result<(), AppError> {
    create_dir_all("images")?;
    let response = ureq::get(url)
        .call()
        .map_err(|e| AppError::Network(e.to_string()))?;

    if response.status() != 200 {
        return Err(AppError::Http(response.status()));
    }

    let mut reader = response.into_reader();
    let file_path = format!("images/dog_{}.jpg", index);
    let mut file = File::create(&file_path)?;
    copy(&mut reader, &mut file)?;
    println!("💾 Saved image to {}", file_path);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Fetcher");
    println!("=================\n");

    for i in 1..=5 {
        println!("Fetching random dog image #{}", i);
        match fetch_random_dog_image() {
            Ok(dog_image) => {
                println!("✅ Success!");
                println!("🖼️ Image URL: {}", dog_image.message);
                println!("📊 Status: {}", dog_image.status);
                if let Err(e) = download_image(&dog_image.message, i) {
                    println!("❌ Download error: {}", e);
                }
            }
            Err(e) => println!("❌ Error: {}", e),
        }
        println!();
    }
    Ok(())
}
