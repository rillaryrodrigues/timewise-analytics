// src/main.rs

use timewise_analytics::{regressao_linear, r2, mse, previsao, ler_csv};

fn main() {
    let dados = ler_csv("dados.csv").expect("Erro ao ler o CSV");

    let (m, b) = regressao_linear(&dados);
    println!("Equação da reta: y = {:.2}x + {:.2}", m, b);

    println!("R² = {:.4}", r2(&dados, m, b));
    println!("MSE = {:.4}", mse(&dados, m, b));

    let futuro_x = 10.0;
    println!("Previsão para x = {}: y = {:.2}", futuro_x, previsao(m, b, futuro_x));
}
