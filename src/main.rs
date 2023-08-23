extern crate reqwest;
extern crate scraper;

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
    let response = client.get(api_url)
        .query(&[("titulo", &search_term), ("searchTerm", &search_term)])
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await?;

    if response.status().is_success() {
        let data = response.text().await?;
        println!("Dados recebidos: {:?}", data);

        let document = Html::parse_document(&data);

        // Agora você pode usar o scraper para selecionar e processar elementos do HTML.
        // Exemplo de seleção de elementos por classe:
        let title_selector = Selector::parse(".title").unwrap();
        for element in document.select(&title_selector) {
            println!("Título da Página: {}", element.text().collect::<String>());
        }

        // Exemplo de seleção de elementos <a> (links):
        let link_selector = Selector::parse("a").unwrap();
        for element in document.select(&link_selector) {
            let link_text = element.text().collect::<String>();
            let link_href = element.value().attr("href").unwrap_or("");
            println!("Link: {} - {}", link_text, link_href);
        }
    } else {
        println!("Erro ao fazer a requisição: {}", response.status());
    }

    Ok(())
}
