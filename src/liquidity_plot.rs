use plotters::coord::Shift;
use plotters::prelude::*;

#[allow(unused_variables)]
pub fn plot(v: Vec<Vec<f32>>, market_names: Vec<String>, file_name: &str, caption: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dir = "plots-output";
    let filepath = format!("{}/{}.png", dir, file_name);
    let root = BitMapBackend::new(&filepath, (1280, 960)).into_drawing_area();

    let child_drawing_areas = root.split_evenly((1, 4));

    for (area,i) in child_drawing_areas.into_iter().zip(0..) {
        plot_subplot(area, v[i].clone(), market_names[i].clone())?;
    }
    Ok(())
}

fn plot_subplot(root: DrawingArea<BitMapBackend, Shift>, v: Vec<f32>, market_name: String) -> Result<(), Box<dyn std::error::Error>>{
    root.fill(&WHITE)?;

    let el_max = v.iter().copied().fold(f32::NAN, f32::max);

    let mut chart = ChartBuilder::on(&root)
        .caption(market_name,
                 ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(60)
        .y_label_area_size(60)
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