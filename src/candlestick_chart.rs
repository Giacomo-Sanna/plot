use std::slice::Windows;
use plotters::prelude::*;
use std::error::Error;
use crate::helpers;

pub(crate) const GREEN: RGBColor = RGBColor(98, 209, 61);
pub(crate) const RED: RGBColor = RGBColor(209, 61, 61);

#[allow(dead_code)]
pub fn plot_image(v: Vec<f32>, candle_size: usize, file_name: &str, caption: &str) {
    let filepath = helpers::get_file_path(file_name);
    let root = BitMapBackend::new(&filepath, (helpers::chart::WIDTH, helpers::chart::HEIGHT));
    plot(&v, candle_size, caption, root,
         (helpers::chart::LABEL_AREA_SIZE, helpers::chart::LABEL_AREA_SIZE), helpers::chart::MARGIN, helpers::chart::WIDTH, helpers::chart::DEFAULT_FONT,
         None, None).expect("ERROR: Unable to plot image!");
    println!("Candlestick chart has been saved to {}", &filepath);
}

pub fn plot<'a, DB: DrawingBackend + 'a>(v: &[f32], candle_size: usize, caption: &str, backend: DB,
                                         label_area_size: (u32, u32), margin: u32, width: u32, font: (&str, u32),
                                         custom_candle_start_index: Option<usize>, custom_y_range: Option<(f32, f32)>)
    -> Result<(), Box<dyn Error + 'a>> {
    if v.is_empty() {
        return Err("ERROR: Vector is empty!".into());
    }

    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;

    let el_max = helpers::f32_max(&v);
    let el_min = helpers::f32_min(&v);
    let data = parse_data(&v, candle_size, custom_candle_start_index);

    let (x_start, x_end) = (data[0].0 - candle_size as isize, data[data.len() - 1].0 + candle_size as isize);

    let (y_start, y_end) = match custom_y_range {
        None => (el_min * 0.9, el_max * 1.05),
        Some((y_min, y_max)) => (y_min, y_max)
    };

    // Basic chart configuration
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(label_area_size.0)
        .y_label_area_size(label_area_size.1)
        .margin(margin)
        .caption(
            caption,
            font.into_font(),
        )
        .build_cartesian_2d(x_start..x_end, (y_start)..(y_end))?;

    chart
        .configure_mesh()
        .light_line_style(&WHITE)
        .draw()?;

    chart
        .draw_series(data.iter().filter(|x| x.1 != x.4)
            .map(|x| {
            CandleStick::new(
                x.0,
                x.1,
                x.2,
                x.3,
                x.4,
                GREEN.filled(),
                RED.filled(),
                ((width - label_area_size.0) as f32
                    / (data.len() as f32 + 2.)).floor() as u32,
            )
        }))?;
    chart
        .draw_series(data.iter().filter(|x| x.1 == x.4)
            .map(|x| {
                CandleStick::new(
                    x.0,
                    x.1,
                    x.2,
                    x.3,
                    x.4,
                    GREEN,
                    RED,
                    ((width - label_area_size.0) as f32
                        / (data.len() as f32 + 2.)).floor() as u32,
                )
            }))?;

    root.present()?;

    Ok(())
}

pub fn parse_data(v: &[f32], candle_size: usize, custom_candle_start_index: Option<usize>) -> Vec<(isize, f32, f32, f32, f32)> {
    fn parse_data_inner(data: Windows<f32>, candle_size: usize, candle_start_index: usize) -> Vec<(isize, f32, f32, f32, f32)> {
        data.enumerate()
            .filter(|(i, _)| i % candle_size == 0)
            .enumerate()
            .map(|(i, (_, v))| (
                (candle_start_index + candle_size*i) as isize,
                 *v.first().unwrap(),
                 helpers::f32_max(v),
                 helpers::f32_min(v),
                 *v.last().unwrap())
            )
            .collect::<Vec<_>>()
    }

    let candle_start_index = match custom_candle_start_index {
        Some(i) => i,
        None => 1,
    };
    return if v.len() < candle_size {
        parse_data_inner(v.windows(v.len()), candle_size, candle_start_index)
    } else {
        let new_el = vec![*v.last().unwrap(); candle_size];
        let new_v = [v, &new_el].concat();
        parse_data_inner(new_v.windows(candle_size + 1), candle_size, candle_start_index)
    };
}