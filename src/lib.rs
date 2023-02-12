pub mod base_chart;
pub mod candlestick_chart;
pub mod bar_chart;
pub mod helpers;
pub mod interactive_candlestick_chart;
pub mod interactive_chart;
pub mod interactive_bar_chart;

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
        println!("candlestick-chart");
        let v = vec![1.5, 4., 2., 5., 10., 12., 3.];
        base_chart::plot_single_series_image(v, "test_base_chart", "Base chart");
    }

    #[test]
    fn test_base_plot_multiple_series() {
        let len: u16 = 10;
        let mut v = vec![];
        for i in 1..len {
            v.push((i..(len + i)).map(f32::from).collect());
        }

        base_chart::plot_multiple_series_image(v, "test_base_chart_multiple_series", "Chart with multiple series");
    }

    #[test]
    fn test_plot_candles() {
        // let v = vec![1.5, 4., 2., 5., 10., 12., 3.];
        let v = helpers::generate_data_series(100., 1000, -0.0985, 0.1);
        candlestick_chart::plot_image(v, 10, "test_candlestick_chart", "Candlestick chart");
    }

    #[test]
    fn test_liquidity_plot() {
        let v: Vec<Vec<f32>> = vec![
            vec![1., 2., 3., 4.],
            vec![7., 5., 3., 1.],
            vec![10., 5., 6., 3.],
            vec![3., 8., 5., 2.]];

        let captions: Vec<String> = (0..v.len()).map(|i| format!("Market {}", i + 1)).collect();

        bar_chart::plot_image(vec![&v[0], &v[1], &v[2], &v[3]], captions, "test_bar_chart");
    }
}
