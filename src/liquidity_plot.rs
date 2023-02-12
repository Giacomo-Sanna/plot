use plotters::coord::Shift;
use plotters::prelude::*;
use std::error::Error;
use crate::helpers;

pub fn plot_image(v: Vec<&[f32]>, captions: Vec<String>, file_name: &str) {
    let filepath = helpers::get_file_path(file_name);
    let root = BitMapBackend::new(&filepath, (helpers::graph::WIDTH, helpers::graph::HEIGHT));
    plot(v, captions, root, None, None,
         (helpers::graph::LABEL_AREA_SIZE, helpers::graph::LABEL_AREA_SIZE), helpers::graph::MARGIN, helpers::graph::DEFAULT_FONT)
        .expect("ERROR: Unable to plot image!");
    println!("Liquidity chart has been saved to {}", &filepath);
}

pub fn plot<'a, DB: DrawingBackend + 'a>(v: Vec<&[f32]>, captions: Vec<String>, backend: DB,
                                         custom_x_start_index: Option<usize>, custom_y_range: Option<(f32, f32)>,
                                         label_area_size: (u32, u32), margin: u32, font: (&str, u32)) -> Result<(), Box<dyn Error + 'a>> {
    let contains_empty = v.iter().fold(false, |prev, v| prev || v.is_empty());
    if v.is_empty() || contains_empty {
        return Err("ERROR: Vector is empty or contains an empty element!".into());
    }
    if v.len() != captions.len() {
        return Err("ERROR: Vector of market names must be the same length as the vector of vectors!".into());
    }

    let root = backend.into_drawing_area();

    let child_drawing_areas = root.split_evenly((1, v.len()));

    let x_start_index = match custom_x_start_index {
        None => 0,
        Some(index) => index
    };

    for (area, i) in child_drawing_areas.into_iter().zip(0..) {
        plot_subplot(area, &v[i], &captions[i], x_start_index, custom_y_range, None, label_area_size, margin, font)?;
    }
    Ok(())
}

pub(crate) fn plot_subplot<'a, DB: DrawingBackend + 'a>(root: DrawingArea<DB, Shift>, v: &[f32], caption: &String,
                                             x_start_index: usize, custom_y_range: Option<(f32, f32)>, bar_margin: Option<u32>,
                                             label_area_size: (u32, u32), margin: u32, font: (&str, u32)) -> Result<(), Box<dyn Error + 'a>> {
    root.fill(&WHITE)?;

    let el_max = helpers::f32_max(&v);

    let (x_start, x_end) = (x_start_index as f32, (x_start_index + v.len()) as f32);

    let (y_start, y_end) = match custom_y_range {
        Some((start, end)) => (start, end),
        None => {
            (0., el_max * 1.05)
        }
    };

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, font.into_font())
        .margin(margin)
        .x_label_area_size(label_area_size.0)
        .y_label_area_size(label_area_size.1)
        .build_cartesian_2d((x_start)..(x_end), (y_start)..(y_end))?;

    chart.configure_mesh().draw()?;

    let gradient = colorous::COOL;
    let n = v.len();
    let bar_margin = match bar_margin {
        Some(m) => m,
        None => 2,
    };

    chart.draw_series(v.iter().enumerate().map(|(x, y)| {
        let color = gradient.eval_rational(x, n);
        let mut bar = Rectangle::new(
            [((x + x_start_index) as f32, 0.), ((x + x_start_index) as f32 + 1., *y)],
            RGBColor(color.r, color.g, color.b).filled());
        bar.set_margin(0, 0, bar_margin, bar_margin);
        bar
    }))?;

    Ok(())
}