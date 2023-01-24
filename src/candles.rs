use std::slice::Windows;
use plotters::prelude::*;

#[allow(dead_code)]
pub fn plot(v: Vec<f32>, candle_size: usize, file_name: &str, caption: &str) {
    let dir = "plots-output";
    let filepath = format!("{}/{}.png", dir, file_name);
    let root = BitMapBackend::new(&filepath, (1280, 960)).into_drawing_area();
    root.fill(&WHITE).expect("Error filling background.");

    let el_max = v.iter().copied().fold(f32::NAN, f32::max);
    let el_min = v.iter().copied().fold(f32::NAN, f32::min);
    let data = parse_data(v, candle_size);

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
            caption,
            ("sans-serif", 50.0).into_font(),
        )
        .build_cartesian_2d(start_date..end_date, (el_min * 0.1)..(el_max * 1.2))
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
                x.2,
                x.3,
                x.4,
                RGBColor(98, 209, 61).filled(),
                RGBColor(209, 61, 61).filled(),
                (1220. / (data.len() as f32 + 2.)).floor() as u32,
            )
        }))
        .unwrap();

    root.present().expect(&format!("Unable to write result to file please make sure directory '{}' exists under the current dir", &dir));

    println!("Plot has been saved to {}", &filepath);
}

pub fn parse_data(v: Vec<f32>, candle_size: usize) -> Vec<(usize, f32, f32, f32, f32)> {

    fn parse_data_inner(data: Windows<f32>, candle_size: usize) -> Vec<(usize, f32, f32, f32, f32)> {
        data.enumerate()
            .filter(|(i, _)| i % candle_size == 0)
            .enumerate()
            .map(|(i, (_, v))| (
                ((i + 1),
                 *v.first().unwrap(),
                 v.iter().copied().fold(f32::NAN, f32::max),
                 v.iter().copied().fold(f32::NAN, f32::min),
                 *v.last().unwrap())
            ))
            .collect::<Vec<_>>()
    }

    return if v.len() < candle_size {
        parse_data_inner(v.windows(v.len()), candle_size)
    } else {
        let last = *v.last().unwrap();
        let to_add = v.len() % candle_size;
        let new_v = [v, vec![last; to_add]].concat();
        // println!("Vector: {:?}", new_v);
        // println!("Window: {:?}", new_v.windows(candle_size + 1));
        parse_data_inner(new_v.windows(candle_size + 1), candle_size)
    }
}