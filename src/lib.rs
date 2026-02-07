//! Biblioteca principal do Minigrep.
//!
//! Este módulo contém a lógica de configuração e busca do programa.
//! 
//! # Licença
//! Este projeto é uma adaptação do tutorial do livro "The Rust Programming Language".
//! Código licenciado sob MIT/Apache 2.0, seguindo as licenças do Rust e do livro.

use std::{error::Error, fs, env};


/// Estrutura que armazena a configuração do programa.
/// 
/// Contém o termo de busca, o caminho do arquivo e argumentos opcionais.
pub struct Config {
    /// Termo a ser buscado no arquivo.
    pub query: String,
    /// Caminho do arquivo onde a busca será realizada.
    pub file_path: String,
    /// Argumentos opcionais (ex: busca case-insensitive).
    pub ignore_case: bool
}


impl Config {
    /// Constrói uma nova configuração a partir dos argumentos da linha de comando.
    /// 
    /// # Argumentos
    /// * `args` - Slice contendo os argumentos da linha de comando
    /// 
    /// # Retorno
    /// Retorna `Ok(Config)` se os argumentos forem válidos, ou `Err` com mensagem de erro.
    /// 
    /// # Erros
    /// Retorna erro se não houver argumentos suficientes (mínimo: query e file_path).
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        
        
        let ignore_case = env::var("IGNORE_CASE").is_ok();
    

        Ok(Config {query, file_path, ignore_case})
    }

    /// Executa a busca no arquivo configurado.
    /// 
    /// Lê o conteúdo do arquivo e realiza a busca pelo termo,
    /// imprimindo as linhas que contêm o termo buscado.
    /// 
    /// # Retorno
    /// Retorna `Ok(())` em caso de sucesso, ou `Err` com o erro ocorrido.
    pub fn run(&self) -> Result<(), Box<dyn Error>>{
        let contents = fs::read_to_string(&self.file_path)?;

        let results = if self.ignore_case {
            search_case_insesitive(&self.query, &contents)
        } else {
            search(&self.query, &contents)
        };
        
        for line in results {
            println!("{line}");
        }
    
        Ok(())
    }

    
}

/// Realiza uma busca case-sensitive no conteúdo fornecido.
/// 
/// # Argumentos
/// * `query` - Termo a ser buscado
/// * `contents` - Conteúdo onde a busca será realizada
/// 
/// # Retorno
/// Retorna um vetor com as linhas que contêm o termo buscado.
pub fn search<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Realiza uma busca case-insensitive no conteúdo fornecido.
/// 
/// Converte tanto o termo quanto cada linha para minúsculas antes de comparar.
/// 
/// # Argumentos
/// * `query` - Termo a ser buscado (será convertido para minúsculas)
/// * `contents` - Conteúdo onde a busca será realizada
/// 
/// # Retorno
/// Retorna um vetor com as linhas originais que contêm o termo buscado.
pub fn search_case_insesitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
Trust me.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

        #[test]
        fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insesitive(query, contents)
        );
    }

}

