use plotters::coord::Shift;
use plotters::prelude::*;
use crate::helpers;

#[allow(unused_variables)]
pub fn plot(v: Vec<Vec<f32>>, market_names: Vec<String>, file_name: &str, caption: &str) -> Result<(), Box<dyn std::error::Error>> {
    let contains_empty = v.iter().fold(false, |prev, v| prev || v.is_empty());
    if v.is_empty() || contains_empty {
        return Err("ERROR: Vector is empty or contains an empty element!".into());
    }

    if v.len() != market_names.len() {
        return Err("ERROR: Vector of market names must be the same length as the vector of vectors!".into());
    }

    let filepath = helpers::get_file_path(file_name);

    let root = BitMapBackend::new(&filepath, (helpers::graph::WIDTH, helpers::graph::HEIGHT)).into_drawing_area();

    let child_drawing_areas = root.split_evenly((1, v.len()));

    for (area,i) in child_drawing_areas.into_iter().zip(0..) {
        plot_subplot(area, v[i].clone(), market_names[i].clone())?;
    }
    Ok(())
}

fn plot_subplot(root: DrawingArea<BitMapBackend, Shift>, v: Vec<f32>, market_name: String) -> Result<(), Box<dyn std::error::Error>>{
    root.fill(&WHITE)?;

    let el_max =  helpers::f32_max(&v);

    let mut chart = ChartBuilder::on(&root)
        .caption(market_name,
                 helpers::graph::DEFAULT_FONT.into_font())
        .margin(5)
        .x_label_area_size(helpers::graph::LABEL_AREA_SIZE)
        .y_label_area_size(helpers::graph::LABEL_AREA_SIZE)
        .build_cartesian_2d((0.)..(v.len() as f32), (0.)..(el_max))?;

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