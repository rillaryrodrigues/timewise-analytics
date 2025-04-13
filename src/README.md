# 📊 TimeWise Analytics - Análise de Séries Temporais em Rust

Este projeto é uma ferramenta interna desenvolvida para a **TimeWise Analytics**, com o objetivo de realizar análises de séries temporais de forma rápida, precisa e eficiente utilizando a linguagem **Rust**, sem dependências externas para cálculos matemáticos.

## 🚀 Funcionalidades

- 📂 Leitura de dados de séries temporais a partir de arquivos `.csv`
- 📈 Implementação manual de **regressão linear** (sem uso de crates externos)
- 🧮 Cálculo de métricas estatísticas:
  - Coeficiente de determinação (**R²**)
  - Erro quadrático médio (**MSE**)
- 📉 Previsões com base no modelo de regressão linear treinado
- ✅ Estrutura pronta para testes unitários e expansão futura

## 📌 Estrutura

- `src/main.rs`: Código-fonte principal com funções de leitura, análise, previsão e métricas
- `dados.csv`: Exemplo de dados usados na regressão
- `Cargo.toml`: Gerenciador de pacotes do projeto Rust

## 🛠 Como Executar

1. Instale o Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2. No terminal insira:
   cargo run