use plotters::prelude::*;

#[allow(dead_code)]
pub fn run(v: Vec<f32>, file_name: &str) {
    let dir = "plots-output";
    let filepath = format!("{}/{}.png", dir, file_name);
    let root = BitMapBackend::new(&filepath, (1280, 960)).into_drawing_area();
    root.fill(&WHITE).expect("Error filling background.");

    let el_max = v.iter().copied().fold(f32::NAN, f32::max);
    let el_min = v.iter().copied().fold(f32::NAN, f32::min);
    let data = parse_data(v);

    // Get date range
    let (start_date, end_date) = (
        data[0].0 - 1,
        data[data.len() - 1].0 + 1,
    );

    // Basic chart configuration
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .caption(
            "Candles",
            ("sans-serif", 50.0).into_font(),
        )
        .build_cartesian_2d(start_date..end_date, (el_min*0.1)..(el_max*1.2))
        .unwrap();

    chart
        .configure_mesh()
        .light_line_style(&WHITE)
        .draw()
        .unwrap();

    chart
        .draw_series(data.iter().map(|x| {
            CandleStick::new(
                x.0,
                 x.1,
                f32::max(x.1, x.2),
                f32::min(x.1, x.2),
                x.2,
                RGBColor(98, 209, 61).filled(),
                RGBColor(209, 61, 61), // RGBColor(209, 61, 61).filled(),
                (1000. / (data.len() as f32 + 1.)).floor() as u32,
            )
        }))
        .unwrap();

    root.present().expect(&format!("Unable to write result to file please make sure directory '{}' exists under the current dir", &dir));

    println!("Plot has been saved to {}", &filepath);
}

fn parse_data(v: Vec<f32>) -> Vec<(usize, f32, f32, f32, f32)> {
    v.iter()
        .zip(v.iter().skip(1))
        .enumerate()
        // (x, open, high, low, close)
        .map(|(i, x)| (i+1, *x.0, f32::max(*x.0, *x.1), f32::min(*x.0, *x.1), *x.1))
        .collect::<Vec<_>>()
}


#[cfg(test)]
mod test{

    #[test]
    fn test_plot_candles() {
        let v = vec![1.5, 4., 2., 5., 10., 12., 3.];
        println!("A: {:?}", super::parse_data(v));
        let v = vec![1.5, 4., 2., 5., 10., 12., 3.];
        super::run(v, "test");
    }
}