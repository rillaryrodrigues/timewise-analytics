// tests/tests.rs
use timewise_analytics::{regressao_linear, r2, mse, previsao};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regressao_linear() {
        let dados = vec![(1.0, 2.0), (2.0, 4.0), (3.0, 6.0)];
        let (m, b) = regressao_linear(&dados);
        assert!((m - 2.0).abs() < 0.01);
        assert!((b - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_r2() {
        let dados = vec![(1.0, 2.0), (2.0, 4.0), (3.0, 6.0)];
        let (m, b) = regressao_linear(&dados);
        let r2_value = r2(&dados, m, b);
        assert!((r2_value - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_mse() {
        let dados = vec![(1.0, 2.0), (2.0, 4.0), (3.0, 6.0)];
        let (m, b) = regressao_linear(&dados);
        let mse_value = mse(&dados, m, b);
        assert!(mse_value < 0.01);
    }

    #[test]
    fn test_previsao() {
        let (m, b) = (2.0, 0.0);
        let previsao_value = previsao(m, b, 5.0);
        assert!((previsao_value - 10.0).abs() < 0.01);
    }
}
