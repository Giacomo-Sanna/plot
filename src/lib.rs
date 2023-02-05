pub mod base_plot;
pub mod candles;
pub mod liquidity_plot;
pub mod helpers;
pub mod interactive_candles;

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
        base_plot::plot_single_series_image(v, "test_base_plot", "base plot");
    }

    #[test]
    fn test_base_plot_multiple_series() {
        let len: u16 = 10;
        let mut v = vec![];
        for i in 1..len {
            v.push((i..(len + i)).map(f32::from).collect());
        }

        base_plot::plot_multiple_series_image(v, "test_base_plot_multiple_series", "Liquidità");
    }

    #[test]
    fn test_plot_candles() {
        // let v = vec![1.5, 4., 2., 5., 10., 12., 3.];
        let v = helpers::generate_data_series(100., 1000, -0.0985, 0.1);
        candles::plot_image(v, 10, "test_plot_candles", "Candles");
    }

    #[test]
    fn test_liquidity_plot() {
        let v: Vec<Vec<f32>> = vec![
            vec![1., 2., 3., 4.],
            vec![7., 5., 3., 1.],
            vec![10., 5., 6., 3.],
            vec![3., 8., 5., 2.]];

        let market_names: Vec<String> = (0..v.len()).map(|i| format!("Market {}", i + 1)).collect();

        liquidity_plot::plot_image(v, market_names, "test_plot_liquidity");
    }
}
