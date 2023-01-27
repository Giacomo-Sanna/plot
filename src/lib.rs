pub mod base_plot;
mod lissajous_curve;
mod candles;
mod liquidity_plot;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_plot() {
        let v = vec![1.5, 4., 2., 5., 10., 12., 3.];
        base_plot::plot(v, "test_base_plot", "base plot").expect("TODO: panic message");
    }

    #[test]
    fn test_base_plot_multiple_series() {
        let len: u16 = 10;
        let mut v = vec![];
        for i in 1..len {
            v.push((i..(len + i)).map(f32::from).collect());
        }

        base_plot::plot_multiple_series(v, "test_base_plot_multiple_series", "base plot multiple series").expect("TODO: panic message");
    }

    #[test]
    fn test_plot_candles() {
        let v = vec![1.5, 4., 2., 5., 10., 12., 3.];
        candles::plot(v, 2, "test_plot_candles", "Candles");
    }

    #[test]
    fn test_liquidity_plot() {
        let v: Vec<Vec<f32>> = vec![
            vec![1., 2., 3., 4.],
            vec![7., 5., 3., 1.],
            vec![10., 5., 6., 3.],
            vec![3., 8., 5., 2.],];

        let market_names: Vec<String> = (0..v.len()).map(|i| format!("Market {}", i)).collect();

        liquidity_plot::plot(v, market_names, "test_plot_liquidity", "Liquidity").expect("TODO: panic message");
    }

}
