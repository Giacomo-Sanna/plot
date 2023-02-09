use plotters::coord::Shift;
use plotters::prelude::*;
use std::error::Error;
use crate::helpers;

pub fn plot_image(v: Vec<&[f32]>, captions: Vec<String>, file_name: &str) {
    let filepath = helpers::get_file_path(file_name);
    let root = BitMapBackend::new(&filepath, (helpers::graph::WIDTH, helpers::graph::HEIGHT));
    plot(v, captions, root, None).expect("ERROR: Unable to plot image!");
    println!("Liquidity chart has been saved to {}", &filepath);
}

pub fn plot<'a, DB: DrawingBackend + 'a>(v: Vec<&[f32]>, captions: Vec<String>, backend: DB, custom_y_range: Option<(f32, f32)>) -> Result<(), Box<dyn Error + 'a>> {
    let contains_empty = v.iter().fold(false, |prev, v| prev || v.is_empty());
    if v.is_empty() || contains_empty {
        return Err("ERROR: Vector is empty or contains an empty element!".into());
    }
    if v.len() != captions.len() {
        return Err("ERROR: Vector of market names must be the same length as the vector of vectors!".into());
    }

    let root = backend.into_drawing_area();

    let child_drawing_areas = root.split_evenly((1, v.len()));

    for (area, i) in child_drawing_areas.into_iter().zip(0..) {
        plot_subplot(area, &v[i], &captions[i], custom_y_range)?;
    }
    Ok(())
}

fn plot_subplot<'a, DB: DrawingBackend + 'a>(root: DrawingArea<DB, Shift>, v: &[f32], caption: &String, custom_y_range: Option<(f32, f32)>) -> Result<(), Box<dyn Error + 'a>> {
    root.fill(&WHITE)?;

    let el_max = helpers::f32_max(&v);

    let (y_start, y_end) = match custom_y_range {
        Some((start, end)) => (start, end),
        None => { (0., el_max)
        }
    };

    let mut chart = ChartBuilder::on(&root)
        .caption(caption,
                 helpers::graph::DEFAULT_FONT.into_font())
        .margin(5)
        .x_label_area_size(helpers::graph::LABEL_AREA_SIZE)
        .y_label_area_size(helpers::graph::LABEL_AREA_SIZE)
        .build_cartesian_2d((0.)..( v.len() as f32), (y_start)..(y_end))?;

    chart.configure_mesh().draw()?;

    let gradient = colorous::SINEBOW;
    let n = v.len();

    chart.draw_series(v.iter().enumerate().map(|(x, y)| {
        let color = gradient.eval_rational(x, n);
        let mut bar = Rectangle::new([(x as f32, 0.), (x as f32 + 1., *y)], RGBColor(color.r, color.g, color.b).filled());
        bar.set_margin(0, 0, 5, 5);
        bar
    }))?;

    Ok(())
}