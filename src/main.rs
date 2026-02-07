//! Minigrep - Uma implementação simplificada do comando grep.
//!
//! Este projeto é uma adaptação do tutorial do livro "The Rust Programming Language".
//! Código licenciado sob MIT/Apache 2.0, seguindo as licenças do Rust e do livro.

use std::{
    env, process
};

use minigrep::Config;

/// Função principal do programa.
/// 
/// Responsável por:
/// - Coletar os argumentos da linha de comando
/// - Construir a configuração a partir dos argumentos
/// - Executar a busca no arquivo especificado
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    if let Err(e) = config.run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    
}

