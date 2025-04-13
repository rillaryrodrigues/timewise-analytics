// src/modelo.rs

// Função para ler os dados do arquivo CSV
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn ler_csv(path: &str) -> Result<Vec<(f64, f64)>, io::Error> {
    let file = File::open(path)?; // Abre o arquivo CSV
    let reader = BufReader::new(file);
    let mut dados = Vec::new();

    for linha in reader.lines() {
        let linha = linha?; // Lê cada linha do arquivo
        let partes: Vec<&str> = linha.trim().split(',').collect();
        if partes.len() == 2 {
            if let (Ok(x), Ok(y)) = (partes[0].parse::<f64>(), partes[1].parse::<f64>()) {
                dados.push((x, y)); // Adiciona o par (x, y) ao vetor de dados
            }
        }
    }

    Ok(dados) // Retorna os dados lidos
}

// Função para calcular a regressão linear (coeficiente angular m e o intercepto b)
pub fn regressao_linear(dados: &[(f64, f64)]) -> (f64, f64) {
    let n = dados.len() as f64;
    let soma_x: f64 = dados.iter().map(|(x, _)| x).sum();
    let soma_y: f64 = dados.iter().map(|(_, y)| y).sum();
    let soma_xy: f64 = dados.iter().map(|(x, y)| x * y).sum();
    let soma_x2: f64 = dados.iter().map(|(x, _)| x * x).sum();

    let m = (n * soma_xy - soma_x * soma_y) / (n * soma_x2 - soma_x * soma_x); // Cálculo do coeficiente angular
    let b = (soma_y - m * soma_x) / n; // Cálculo do intercepto
    (m, b) // Retorna os valores de m e b
}

// Função para calcular o R² (coeficiente de determinação)
pub fn r2(dados: &[(f64, f64)], m: f64, b: f64) -> f64 {
    let media_y: f64 = dados.iter().map(|(_, y)| y).sum::<f64>() / dados.len() as f64; // Média dos valores y
    let ss_total: f64 = dados.iter().map(|(_, y)| (y - media_y).powi(2)).sum(); // Soma dos quadrados totais
    let ss_res: f64 = dados.iter().map(|(x, y)| (y - (m * x + b)).powi(2)).sum(); // Soma dos quadrados dos resíduos
    1.0 - ss_res / ss_total // Cálculo do R²
}

// Função para calcular o erro quadrático médio (MSE)
pub fn mse(dados: &[(f64, f64)], m: f64, b: f64) -> f64 {
    dados.iter().map(|(x, y)| (y - (m * x + b)).powi(2)).sum::<f64>() / dados.len() as f64 // Cálculo do MSE
}

// Função para fazer previsões usando a equação da reta
pub fn previsao(m: f64, b: f64, x: f64) -> f64 {
    m * x + b // Cálculo da previsão para um valor de x
}
