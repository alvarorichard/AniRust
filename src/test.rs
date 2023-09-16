extern crate reqwest;
extern crate scraper;

use lazy_regex::regex_captures;
use std::io;

use scraper::{Html, Selector};
use tokio;

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Por favor, insira o nome do anime:");

    let mut anime_name = String::new();
    io::stdin().read_line(&mut anime_name)?;
    let anime_name = anime_name.trim().to_string();

    // Formatar o nome do anime para ser URL-friendly (ex: substituir espaços por -)
    let search_term = anime_name.replace(" ", "-").to_lowercase();

    let api_url = "https://betteranime.net/pesquisa";


    let client = reqwest::Client::new();
    let response = client
        .get(api_url)
        .query(&[("titulo", &search_term), ("searchTerm", &search_term)])
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await?;

    if response.status().is_success() {
        let data = response.text().await?;
        let found = regex_captures!(r#""(https:\/\/betteranime\.net\/anime\/([\w]+)\/([\w-]+))""#, &data);
        println!("{:?}", &found);
    } else {
        println!("Erro ao fazer a requisição: {}", response.status());
    }

    Ok(())
}