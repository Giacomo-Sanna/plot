use plotters::prelude::*;
pub fn plot(v: Vec<f32>, file_name: &str, caption: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dir = "plots-output";
    let filepath = format!("{}/{}.png", dir, file_name);
    let root = BitMapBackend::new(&filepath, (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;

    let el_max = v.iter().copied().fold(f32::NAN, f32::max);
    let el_min = v.iter().copied().fold(f32::NAN, f32::min);

    let mut chart = ChartBuilder::on(&root)
        .caption(caption,
                 ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(0..(v.len()-1), (el_min*0.9)..(el_max*1.1))?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            // (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            v.into_iter().enumerate(),
            &BLUE,
        ))?;
        // .label("y = x^2")
        // .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // chart
    //     .configure_series_labels()
    //     .background_style(&WHITE.mix(0.8))
    //     .border_style(&BLACK)
    //     .draw()?;

    root.present().expect(&format!("Unable to write result to file please make sure directory '{}' exists under the current dir", &dir));

    Ok(())
}

pub fn plot_multiple_series(vec: Vec<Vec<f32>>, file_name: &str, caption: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dir = "plots-output";
    let filepath = format!("{}/{}.png", dir, file_name);
    let root = BitMapBackend::new(&filepath, (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut el_max = f32::NAN;
    let mut el_min = f32::NAN;

    for v in vec.iter() {
        el_max = f32::max(el_max, v.iter().copied().fold(f32::NAN, f32::max));
        el_min = f32::min(el_min, v.iter().copied().fold(f32::NAN, f32::min));
    }
    let max_len = vec.iter().map(|v| v.len()).max().unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(caption,
                 ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(0..(max_len -1), (el_min*0.9)..(el_max*1.1))?;

    chart.configure_mesh().draw()?;

    // for different color schemes see https://docs.rs/colorous/1.0.9/colorous/#colorous
    let gradient = colorous::SINEBOW;
    let n = vec.len();

    for (i, v) in vec.into_iter().enumerate(){
        let cor = gradient.eval_rational(i, n);

        chart
            .draw_series(LineSeries::new(
                // (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                v.into_iter().enumerate(),
                RGBColor(cor.r, cor.g, cor.b),
            ))?;
    }

    root.present().expect(&format!("Unable to write result to file please make sure directory '{}' exists under the current dir", &dir));

    Ok(())
}