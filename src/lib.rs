pub mod base_plot;
mod candles;
mod liquidity_plot;
mod helpers;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_dir() {
        use std::fs;
        fs::create_dir_all(format!("./{}", helpers::graph::DEFAULT_DIR)).expect("ERROR: Unable to create images' directory!");
    }

    #[test]
    fn test_base_plot() {
        let v = vec![1.5, 4., 2., 5., 10., 12., 3.];
        base_plot::plot(v, "test_base_plot", "base plot").expect("TODO: panic message");
        assert!(base_plot::plot(vec![], "error", "base plot").is_err());
    }

    #[test]
    fn test_base_plot_multiple_series() {
        let len: u16 = 10;
        let mut v = vec![];
        for i in 1..len {
            v.push((i..(len + i)).map(f32::from).collect());
        }

        base_plot::plot_multiple_series(v, "test_base_plot_multiple_series", "base plot multiple series").expect("TODO: panic message");
        assert!(base_plot::plot_multiple_series(vec![], "error", "").is_err());
        assert!(base_plot::plot_multiple_series(vec![vec![]], "error", "").is_err());
    }

    #[test]
    fn test_plot_candles() {
        let v = vec![1.5, 4., 2., 5., 10., 12., 3.];
        candles::plot(v, 2, "test_plot_candles", "Candles").expect("TODO: panic message");
        assert!(candles::plot(vec![], 2, "error", "").is_err());
    }

    #[test]
    fn test_liquidity_plot() {
        let v: Vec<Vec<f32>> = vec![
            vec![1., 2., 3., 4.],
            vec![7., 5., 3., 1.],
            vec![10., 5., 6., 3.],
            vec![3., 8., 5., 2.]];

        let market_names: Vec<String> = (0..v.len()).map(|i| format!("Market {}", i)).collect();

        liquidity_plot::plot(v, market_names, "test_plot_liquidity", "Liquidity").expect("TODO: panic message");
        assert!(liquidity_plot::plot(vec![], vec![], "error", "").is_err());
        assert!(liquidity_plot::plot(vec![vec![]], vec![], "error", "").is_err());
        assert!(liquidity_plot::plot(vec![vec![1.], vec![2.]], vec!["".to_string()], "error", "").is_err());
    }

}
