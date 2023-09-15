extern crate reqwest;
use regex::Regex;
use std::io;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Por favor, insira o nome do anime:");

    let mut anime_name = String::new();
    io::stdin().read_line(&mut anime_name)?;
    let anime_name = anime_name.trim().to_string();

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
        let re = Regex::new(r#""(https:\/\/betteranime\.net\/anime\/([\w]+)\/([\w-]+))""#).unwrap();
        let found: Vec<_> = re.captures_iter(&data).collect();

        println!("Opções encontradas:");
        for (i, cap) in found.iter().enumerate() {
            println!("{}: {}", i + 1, &cap[1]);
        }

        println!("Por favor, selecione uma opção:");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice: usize = choice.trim().parse()?;

        if choice > 0 && choice <= found.len() {
            let selected_url = &found[choice - 1][1];
            println!("Você selecionou: {}", selected_url);
            // Aqui você pode adicionar o código para dar play no vídeo
        } else {
            println!("Opção inválida.");
        }
    } else {
        println!("Erro ao fazer a requisição: {}", response.status());
    }

    Ok(())
}
