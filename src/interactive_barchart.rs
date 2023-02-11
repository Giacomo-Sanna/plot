use std::borrow::{BorrowMut};
use plotters::prelude::*;
use std::error::Error;
use plotters::chart::ChartState;
use plotters::coord::types::{RangedCoordf32};
use crate::helpers;
use crate::helpers::interactive_chart::*;
use plotters::backend::BGRXPixel;

pub const INSTRUCTIONS: &str = "Instructions:
  ←/→=Previous/next series
  +/-=Adjust sample rate
  1/2=Adjust start index
  9/0=Adjust end index
  P=Start/Stop
  R=Restart
  <Esc>=Exit
";

pub(crate) fn initialize_buff_chart(buf: &mut helpers::BufferWrapper, v: &[f32], x_start_index: usize) -> Result<ChartState<Cartesian2d<RangedCoordf32, RangedCoordf32>>, Box<dyn Error>> {
    let cs = {
        let root =
            BitMapBackend::<BGRXPixel>::with_buffer_and_format(buf.borrow_mut(), (W as u32, H as u32))?
                .into_drawing_area();

        root.fill(&WHITE)?;

        let el_max = helpers::f32_max(&v);

        let (x_start, x_end) = (x_start_index as f32, (x_start_index + v.len()) as f32);

        let (y_start, y_end) = (0., el_max * 1.05);

        let mut chart = ChartBuilder::on(&root)
            // .caption(caption, font.into_font())
            .margin(MARGIN)
            .x_label_area_size(LABEL)
            .y_label_area_size(LABEL)
            .build_cartesian_2d((x_start)..(x_end), (y_start)..(y_end))?;

        chart.configure_mesh().draw()?;

        let cs = chart.into_chart_state();
        root.present()?;
        cs
    };
    Ok(cs)
}

pub(crate) fn draw_buff_chart(buf: &mut helpers::BufferWrapper, v: &[f32], curr_index: usize, start_index: usize, cs: &ChartState<Cartesian2d<RangedCoordf32, RangedCoordf32>>) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
        buf.borrow_mut(),
        (W as u32, H as u32),
    )?
        .into_drawing_area();
    {
        let mut chart = cs.clone().restore(&root);
        chart.plotting_area().fill(&WHITE)?;

        chart
            .configure_mesh()
            .draw()?;

        let x_start_index = start_index;

        let gradient = colorous::COOL;
        let n = v.len();

        chart.draw_series(v[..curr_index].iter().enumerate().map(|(x, y)| {
            let color = gradient.eval_rational(x, n);
            let mut bar = Rectangle::new(
                [((x + x_start_index) as f32, 0.), ((x + x_start_index) as f32 + 1., *y)],
                RGBColor(color.r, color.g, color.b).filled());
            bar.set_margin(0, 0, 1, 1);
            bar
        }))?;
    }
    root.present()?;
    Ok(())
}

pub(crate) fn get_window_title(v_name: &str, paused: bool, sr: f64, start_index: usize, end_index: usize) -> String {
    let paused_text = if paused { "PAUSED, " } else { "" };
    format!(
        "{} {}, sample rate = {:.1}, start index = {}, end index = {}",
        paused_text, v_name, sr, start_index, end_index
    )
}