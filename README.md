# Minigrep

Uma implementação simplificada do comando `grep` em Rust.

## Sobre o Projeto

Este projeto é uma adaptação do tutorial apresentado no livro **"The Rust Programming Language"** (também conhecido como "The Rust Book"), disponível em [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/).

O objetivo deste projeto é exclusivamente **educacional**, servindo como exercício de aprendizado da linguagem Rust e seus conceitos fundamentais, incluindo:

- Organização de código com módulos
- Tratamento de erros
- Uso de lifetimes
- Testes automatizados
- Leitura de arquivos
- Argumentos de linha de comando

## Uso

```bash
cargo run -- <termo_busca> <arquivo> [opções]
```

### Opções

- `i` - Busca case-insensitive (ignora maiúsculas/minúsculas)

### Exemplos

```bash
# Busca simples
cargo run -- to poem.txt

# Busca ignorando maiúsculas/minúsculas
cargo run -- to poem.txt i
```

## Licença e Copyright

### Rust

Rust é licenciado sob a licença Apache 2.0 e MIT. Para mais informações, consulte:
- [https://www.rust-lang.org/policies/licenses](https://www.rust-lang.org/policies/licenses)

### The Rust Programming Language (Livro)

O livro "The Rust Programming Language" é disponibilizado sob as seguintes licenças:
- O texto está licenciado sob **Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International (CC BY-NC-SA 4.0)**
- O código de exemplo está licenciado sob **MIT/Apache 2.0**

Para mais detalhes, consulte o repositório oficial: [https://github.com/rust-lang/book](https://github.com/rust-lang/book)

## Aviso

Este projeto foi criado apenas para fins de estudo e aprendizado. Não é destinado para uso em produção.

## Uso de IA

Este projeto utilizou ferramentas de IA (GitHub Copilot) **apenas** para:
- Geração dos comentários de documentação no código
- Criação deste arquivo README

**Todo o código-fonte foi escrito manualmente**, seguindo o tutorial do livro "The Rust Programming Language".
