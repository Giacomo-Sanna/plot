pub mod base_plot;
mod graph_image1;
mod graph_image2;
mod lissajous_curve;
mod plot;

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
        println!("A: {:?}", plot::parse_data(v));
        let v = vec![1.5, 4., 2., 5., 10., 12., 3.];
        plot::run(v, "test_plot_candles");
    }
}
