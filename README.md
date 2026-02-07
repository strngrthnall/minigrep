# 🔍 Minigrep

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue?style=for-the-badge)
![Status](https://img.shields.io/badge/Status-Learning%20Project-yellow?style=for-the-badge)

> Uma implementação simplificada do comando `grep` em Rust 🦀

---

## 📚 Sobre o Projeto

Este projeto é uma adaptação do tutorial apresentado no livro **"The Rust Programming Language"** (também conhecido como "The Rust Book"), disponível em [doc.rust-lang.org/book](https://doc.rust-lang.org/book/).

O objetivo deste projeto é exclusivamente **educacional** 🎓, servindo como exercício de aprendizado da linguagem Rust e seus conceitos fundamentais.

### ✨ Conceitos Aprendidos

| Conceito | Descrição |
|----------|-----------|
| 📦 Módulos | Organização de código com módulos |
| ⚠️ Tratamento de Erros | Uso de `Result` e tratamento adequado |
| ⏳ Lifetimes | Gerenciamento de referências |
| 🧪 Testes | Testes automatizados com `#[test]` |
| 📄 I/O | Leitura de arquivos e argumentos CLI |

---

## 🚀 Uso

```bash
cargo run -- <termo_busca> <arquivo> [opções]
```

### ⚙️ Opções

| Opção | Descrição |
|-------|-----------|
| `IGNORE_CASE` | Busca case-insensitive (ignora maiúsculas/minúsculas) |

### 💡 Exemplos

```bash
# 🔎 Busca simples
cargo run -- to poem.txt

# 🔎 Busca ignorando maiúsculas/minúsculas
cargo run -- to poem.txt i
```

---

## 📜 Licença e Copyright

### 🦀 Rust

Rust é licenciado sob a licença **Apache 2.0** e **MIT**.

🔗 [rust-lang.org/policies/licenses](https://www.rust-lang.org/policies/licenses)

### 📖 The Rust Programming Language (Livro)

O livro "The Rust Programming Language" é disponibilizado sob as seguintes licenças:

| Tipo | Licença |
|------|---------|
| 📝 Texto | CC BY-NC-SA 4.0 |
| 💻 Código | MIT / Apache 2.0 |

🔗 [github.com/rust-lang/book](https://github.com/rust-lang/book)

---

## 🤖 Uso de IA

Este projeto utilizou ferramentas de IA (GitHub Copilot) **apenas** para:
- 📝 Geração dos comentários de documentação no código
- 📄 Criação deste arquivo README

> **Todo o código-fonte foi escrito manualmente** ✍️, seguindo o tutorial do livro "The Rust Programming Language".

---

## ⚠️ Aviso

> 🎓 Este projeto foi criado apenas para fins de **estudo e aprendizado**.
> Não é destinado para uso em produção.

---

<p align="center">
  Feito com ❤️ enquanto aprendia Rust 🦀
</p>
