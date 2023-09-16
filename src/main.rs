extern crate reqwest;
use regex::Regex;
use std::io;
use std::process::Command;
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

        if found.is_empty() {
            println!("Nenhum anime encontrado.");
            return Ok(());
        }

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

            // Fazer uma solicitação HTTP para obter a página do anime selecionado
            let episode_page_response = client.get(selected_url)
                .header("User-Agent", "Mozilla/5.0")
                .send()
                .await?;

            if episode_page_response.status().is_success() {
                let episode_page_data = episode_page_response.text().await?;

                // Usar regex para listar os episódios
                let re = Regex::new(r"https:\/\/betteranime\.net\/anime\/[\w]+\/[\w-]+\/[\w]+-(\d+)").unwrap();
                let episodios: Vec<_> = re.captures_iter(&episode_page_data)
                    .map(|cap| format!("Episódio {}", &cap[1]))
                    .collect();

                if episodios.is_empty() {
                    println!("Nenhum episódio encontrado.");
                } else {
                    println!("Episódios encontrados:");
                    for (i, ep) in episodios.iter().enumerate() {
                        println!("{}: {}", i + 1, ep);
                    }

                    println!("Por favor, selecione um episódio:");
                    let mut episode_choice = String::new();
                    io::stdin().read_line(&mut episode_choice)?;
                    let episode_choice: usize = episode_choice.trim().parse()?;

                    if episode_choice > 0 && episode_choice <= episodios.len() {
                        let selected_episode = &episodios[episode_choice - 1];
                        println!("Você selecionou: {}", selected_episode);

                        // Substitua esta regex pela que corresponde ao seu site para criar o URL do episódio
                        let re = Regex::new(r"https:\/\/site\.com\/[\w]+\/episodio-(\d+)").unwrap();
                        let episode_url = re.captures(selected_episode)
                            .map(|cap| cap[0].to_string())
                            .unwrap_or_else(|| "URL do episódio não encontrado".to_string());

                        // Executar o MPV com o URL do episódio
                        let vlc_result = Command::new("vlc")
                            .arg(&episode_url)
                            .spawn();

                        match vlc_result {
                            Ok(mut child) => {
                                // Esperar pelo término do MPV (o programa ficará bloqueado aqui até que o MPV seja encerrado)
                                match child.wait() {
                                    Ok(_) => {
                                        println!("MPV encerrado.");
                                    },
                                    Err(err) => {
                                        eprintln!("Erro ao esperar pelo MPV: {}", err);
                                    }
                                }
                            },
                            Err(err) => {
                                eprintln!("Erro ao iniciar o MPV: {}", err);
                            }
                        }
                    } else {
                        println!("Opção de episódio inválida.");
                    }
                }
            } else {
                println!("Erro ao fazer a requisição para a página do anime: {}", episode_page_response.status());
            }

        } else {
            println!("Opção inválida.");
        }
    } else {
        println!("Erro ao fazer a requisição: {}", response.status());
    }

    Ok(())
}
