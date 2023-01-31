use std::slice::Windows;
use plotters::prelude::*;
use crate::helpers;

#[allow(dead_code)]
pub fn plot(v: Vec<f32>, candle_size: usize, file_name: &str, caption: &str) -> Result<(), Box<dyn std::error::Error>>{
    if v.is_empty() {
        return Err("ERROR: Vector is empty!".into());
    }

    let filepath = helpers::get_file_path(file_name);

    let root = BitMapBackend::new(&filepath, (helpers::graph::WIDTH, helpers::graph::HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;

    let el_max = helpers::f32_max(&v);
    let el_min = helpers::f32_min(&v);
    let data = parse_data(&v, candle_size);

    // Get date range
    let (start_date, end_date) = (
        data[0].0 - 1,
        data[data.len() - 1].0 + 1,
    );

    // Basic chart configuration
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(helpers::graph::LABEL_AREA_SIZE)
        .y_label_area_size(helpers::graph::LABEL_AREA_SIZE)
        .caption(
            caption,
            helpers::graph::DEFAULT_FONT.into_font(),
        )
        .build_cartesian_2d(start_date..end_date, (el_min * 0.1)..(el_max * 1.2))?;

    chart
        .configure_mesh()
        .light_line_style(&WHITE)
        .draw()?;

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
                ((helpers::graph::WIDTH-helpers::graph::LABEL_AREA_SIZE) as f32
                    / (data.len() as f32 + 2.)).floor() as u32,
            )
        }))?;

    root.present().expect(helpers::graph::ERROR_PRESENTING);

    println!("Plot has been saved to {}", &filepath);
    Ok(())
}

pub fn parse_data(v: &[f32], candle_size: usize) -> Vec<(usize, f32, f32, f32, f32)> {

    fn parse_data_inner(data: Windows<f32>, candle_size: usize) -> Vec<(usize, f32, f32, f32, f32)> {
        data.enumerate()
            .filter(|(i, _)| i % candle_size == 0)
            .enumerate()
            .map(|(i, (_, v))| (
                ((i + 1),
                 *v.first().unwrap(),
                 helpers::f32_max(v),
                 helpers::f32_min(v),
                 *v.last().unwrap())
            ))
            .collect::<Vec<_>>()
    }

    return if v.len() < candle_size {
        parse_data_inner(v.windows(v.len()), candle_size)
    } else {
        let new_el = vec![*v.last().unwrap(); v.len() % candle_size];
        let new_v = [v, &new_el].concat();
        // println!("Vector: {:?}", new_v);
        // println!("Window: {:?}", new_v.windows(candle_size + 1));
        parse_data_inner(new_v.windows(candle_size + 1), candle_size)
    }
}