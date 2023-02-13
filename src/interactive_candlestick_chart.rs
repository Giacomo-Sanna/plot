use std::borrow::{BorrowMut};
use plotters::prelude::*;
use std::error::Error;
use plotters::chart::ChartState;
use plotters::coord::types::{RangedCoordf32, RangedCoordi32};
use crate::helpers;
use crate::helpers::interactive_chart::*;
use crate::candlestick_chart;
use plotters::backend::BGRXPixel;


pub const INSTRUCTIONS: &str = "Instructions:
  ←/→=Previous/next series
  ↑/↓=Adjust candle size
  +/-=Adjust sample rate
  1/2=Adjust start index
  9/0=Adjust end index
  P=Start/Stop
  R=Restart
  <Esc>=Exit
";

pub(crate) fn initialize_buff_chart(buf: &mut helpers::BufferWrapper, v: &[f32], candle_size: usize, start_index: usize, caption: &str) -> Result<ChartState<Cartesian2d<RangedCoordi32, RangedCoordf32>>, Box<dyn Error>> {
    let el_max = helpers::f32_max(&v);
    let el_min = helpers::f32_min(&v);

    let custom_candle_start_index: usize = start_index;

    let candles_data = candlestick_chart::parse_data(&v, candle_size, Some(custom_candle_start_index + 1));
    let (start_date, end_date) = (
        (custom_candle_start_index as i32 - candle_size as i32),
        (candles_data[candles_data.len() - 1].0 as i32 + candle_size as i32),
    );

    let cs = {
        let root =
            BitMapBackend::<BGRXPixel>::with_buffer_and_format(buf.borrow_mut(), (W as u32, H as u32))?
                .into_drawing_area();

        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(caption, DEFAULT_FONT.into_font())
            .margin(MARGIN)
            .x_label_area_size(LABEL)
            .y_label_area_size(LABEL)
            .build_cartesian_2d(start_date..end_date, (el_min * 0.9)..(el_max * 1.05))?;

        chart
            .configure_mesh()
            .light_line_style(&WHITE)
            .draw()?;

        let cs = chart.into_chart_state();
        root.present()?;
        cs
    };
    Ok(cs)
}

pub(crate) fn draw_buff_chart(buf: &mut helpers::BufferWrapper, v: &[f32], candle_size: usize, curr_index: usize, start_index: usize, cs: &ChartState<Cartesian2d<RangedCoordi32, RangedCoordf32>>) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
        buf.borrow_mut(),
        (W as u32, H as u32),
    )?.into_drawing_area();

    let mut chart = cs.clone().restore(&root);
    chart.plotting_area().fill(&WHITE)?;

    chart
        .configure_mesh()
        .light_line_style(&WHITE)
        .draw()?;

    let custom_candle_start_index: usize = start_index;

    let data = candlestick_chart::parse_data(&v[..curr_index], candle_size, Some(custom_candle_start_index + 1));
    let n_elements = candlestick_chart::parse_data(&v, candle_size, None).len();

    chart
        .draw_series(data.iter().filter(|x| x.1 != x.4)
            .map(|x| {
            CandleStick::new(
                x.0 as i32,
                x.1,
                x.2,
                x.3,
                x.4,
                candlestick_chart::GREEN.filled(),
                candlestick_chart::RED.filled(),
                ((W as u32 - MARGIN - LABEL) as f32 / (n_elements as f32 + 2.)).floor() as u32,
            )
        }))?;
    chart.draw_series(data.iter().filter(|x| x.1 == x.4)
        .map(|x| {
            CandleStick::new(
                x.0 as i32,
                x.1,
                x.2,
                x.3,
                x.4,
                candlestick_chart::GREEN,
                candlestick_chart::RED,
                ((W as u32 - MARGIN - LABEL) as f32 / (n_elements as f32 + 2.)).floor() as u32,
            )
        }))?;

    root.present()?;
    Ok(())
}

pub(crate) fn get_window_title(paused: bool, candle_size: usize, sr: f64, start_index: usize, end_index: usize) -> String {
    let paused_text = if paused { "PAUSED, " } else { "" };
    format!(
        "{}candle size = {}, sample rate = {:.1}, start index = {}, end index = {}",
        paused_text, candle_size, sr, start_index, end_index
    )
}