use std::error::Error;
use plotters::prelude::*;
use crate::helpers;

pub fn plot_single_series_image(v: Vec<f32>, file_name: &str, caption: &str) {
    let filepath = helpers::get_file_path(file_name);
    let root = BitMapBackend::new(&filepath, (helpers::graph::WIDTH, helpers::graph::HEIGHT));
    plot_single_series(v, caption, root).expect("ERROR: Unable to plot image!");
    println!("Single series chart has been saved to {}", &filepath);
}

pub fn plot_single_series<'a, DB: DrawingBackend + 'a>(v: Vec<f32>, caption: &str, backend: DB) -> Result<(), Box<dyn Error + 'a>> {
    if v.is_empty() {
        return Err("ERROR: Vector is empty!".into());
    }

    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;

    let el_max = helpers::f32_max(&v);
    let el_min = helpers::f32_min(&v);

    let mut chart = ChartBuilder::on(&root)
        .caption(caption,
                 helpers::graph::DEFAULT_FONT.into_font())
        .margin(5)
        .x_label_area_size(helpers::graph::LABEL_AREA_SIZE)
        .y_label_area_size(helpers::graph::LABEL_AREA_SIZE)
        .build_cartesian_2d(0..(v.len() - 1), (el_min * 0.9)..(el_max * 1.1))?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            v.into_iter().enumerate(),
            &BLUE,
        ))?;

    root.present()?;

    Ok(())
}

pub fn plot_multiple_series_image(vec: Vec<Vec<f32>>, file_name: &str, caption: &str) {
    let filepath = helpers::get_file_path(file_name);
    let root = BitMapBackend::new(&filepath, (helpers::graph::WIDTH, helpers::graph::HEIGHT));
    plot_multiple_series(vec, caption, root).expect("ERROR: Unable to plot image!");
    println!("Multiple series chart has been saved to {}", &filepath);
}

pub fn plot_multiple_series<'a, DB: DrawingBackend + 'a>(vec: Vec<Vec<f32>>, caption: &str, backend: DB) -> Result<(), Box<dyn Error + 'a>> {
    let contains_empty = vec.iter().fold(false, |prev, v| prev || v.is_empty());
    if vec.is_empty() || contains_empty {
        return Err("ERROR: Vector is empty or contains an empty element!".into());
    }

    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;

    let mut el_max = f32::NAN;
    let mut el_min = f32::NAN;

    for v in vec.iter() {
        el_max = f32::max(helpers::f32_max(&v), el_max);
        el_min = f32::min(helpers::f32_min(&v), el_min);
    }
    let max_len = vec.iter().map(|v| v.len()).max().unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(caption,
                 helpers::graph::DEFAULT_FONT.into_font())
        .margin(5)
        .x_label_area_size(helpers::graph::LABEL_AREA_SIZE)
        .y_label_area_size(helpers::graph::LABEL_AREA_SIZE)
        .build_cartesian_2d(0..(max_len - 1), (el_min * 0.9)..(el_max * 1.1))?;

    chart.configure_mesh().draw()?;

    // for different color schemes see https://docs.rs/colorous/1.0.9/colorous/#colorous
    let gradient = colorous::SINEBOW;
    let n = vec.len();

    for (i, v) in vec.into_iter().enumerate() {
        let rgb = gradient.eval_rational(i, n);

        chart
            .draw_series(LineSeries::new(
                v.into_iter().enumerate(),
                RGBColor(rgb.r, rgb.g, rgb.b),
            ))?;
    }

    root.present()?;
    Ok(())
}