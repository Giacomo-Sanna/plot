use std::borrow::{Borrow, BorrowMut};
use plotters::prelude::*;
use std::error::Error;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::time::SystemTime;
use plotters::chart::ChartState;
use plotters::coord::types::{RangedCoordf32, RangedCoordi32};
use crate::helpers;
use crate::candles;
use plotters::backend::BGRXPixel;

const W: usize = 800;
const H: usize = 600;
const MARGIN: u32 = 10;
const LABEL: u32 = 30;

const SAMPLE_RATE: f64 = 60.0;

fn initialize_buff_chart(buf: &mut helpers::BufferWrapper, v: &[f32], candle_size: usize, start_index: usize) -> Result<ChartState<Cartesian2d<RangedCoordi32, RangedCoordf32>>, Box<dyn Error>> {
    let el_max = helpers::f32_max(&v);
    let el_min = helpers::f32_min(&v);

    let custom_candle_start_index: usize = (start_index as f32 / candle_size as f32).floor() as usize;

    let candles_data = candles::parse_data(&v, candle_size, Some(custom_candle_start_index + 1));
    let (start_date, end_date) = (
        (custom_candle_start_index as i32 - candle_size as i32) ,
        (candles_data[candles_data.len() - 1].0 as i32 + candle_size as i32),
    );

    let cs = {
        let root =
            BitMapBackend::<BGRXPixel>::with_buffer_and_format(buf.borrow_mut(), (W as u32, H as u32))?
                .into_drawing_area();

        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .margin(MARGIN)
            .x_label_area_size(LABEL)
            .y_label_area_size(LABEL)
            .build_cartesian_2d(start_date..end_date, (el_min * 0.1)..(el_max * 1.05))?;

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

fn draw_buff_chart(buf: &mut helpers::BufferWrapper, v: &[f32], candle_size: usize, curr_index: usize, start_index: usize, cs: &ChartState<Cartesian2d<RangedCoordi32, RangedCoordf32>>) -> Result<(), Box<dyn Error>> {
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
            .light_line_style(&WHITE)
            .draw()?;

        let custom_candle_start_index: usize = (start_index as f32 / candle_size as f32).floor() as usize;

        let data = candles::parse_data(&v[..curr_index], candle_size, Some(custom_candle_start_index + 1));
        let n_elements = candles::parse_data(&v, candle_size, None).len();

        chart
            .draw_series(data.iter().map(|x| {
                CandleStick::new(
                    x.0 as i32,
                    x.1,
                    x.2,
                    x.3,
                    x.4,
                    RGBColor(98, 209, 61).filled(),
                    RGBColor(209, 61, 61).filled(),
                    ((W as u32 - MARGIN - LABEL) as f32 / (n_elements as f32 + 2.)).floor() as u32,
                )
            }))?;
    }
    root.present()?;
    Ok(())
}

fn get_window_title(v_name: &str, paused: bool, candle_size: usize, sr: f64, start_index: usize, end_index: usize) -> String {
    let paused_text = if paused { "PAUSED, " } else { "" };
    format!(
        "{} {}, candle size = {}, sample rate = {:.1}, start index = {}, end index = {}",
        paused_text, v_name, candle_size, sr, start_index, end_index
    )
}

fn initialize_ts(start_ts: SystemTime) -> f64 {
    SystemTime::now()
        .duration_since(start_ts)
        .unwrap()
        .as_secs_f64()
}

pub fn launch_gui(vec: Vec<(&str, Vec<f32>)>, candle_size: usize) -> Result<(), Box<dyn Error>> {
    let mut v_index = 0;
    let mut v = &vec[v_index].1;
    let mut v_name = *&vec[v_index].0;

    println!("Instructions:\n  ←/→=Previous/next series\n  ↑/↓=Adjust candle size\n  +/-=Adjust sample rate\n  1/2=Adjust start index\n  9/0=Adjust end index\n  P=Start/Stop\n  R=Restart\n  <Esc>=Exit\n");

    let mut buf = helpers::BufferWrapper::new(W, H);

    let mut paused = false;
    let mut candle_size = candle_size;
    let mut sample_rate = SAMPLE_RATE;
    let mut start_index = 0;
    let mut end_index = v.len();

    let mut reloading_required = false;
    let mut curr_index = 0;
    let mut last_index_flushed = 0;

    let mut window = Window::new(
        &get_window_title(v_name, paused, candle_size, sample_rate, start_index, end_index),
        W,
        H,
        WindowOptions::default(),
    )?;

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut cs = initialize_buff_chart(&mut buf, &v, candle_size, start_index)?;

    let start_ts = SystemTime::now();
    let mut ts = initialize_ts(start_ts);

    let mut previous_key = Key::Unknown;
    let mut increment = 0;
    let base: usize = 2;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let epoch = initialize_ts(start_ts);

        while ts < epoch && curr_index < end_index
            && !paused {
            ts += 1.0 / sample_rate;
            curr_index += 1;
        }

        if (last_index_flushed < curr_index
            || reloading_required)
            && curr_index > start_index {
            last_index_flushed = curr_index;
            reloading_required = false;
            draw_buff_chart(&mut buf, &v[start_index..end_index], candle_size, curr_index - start_index, start_index, &cs)?;
        }

        let keys = window.get_keys_pressed(KeyRepeat::Yes);
        for key in keys {
            if previous_key == key { increment+=1; } else { increment = 0; }
            previous_key = key;
            match key {
                // increment sample rate
                Key::Equal => {
                    if sample_rate < 100_000.
                    {
                        sample_rate += base.pow(increment) as f64;
                        if sample_rate > 100_000. { sample_rate = 100_000. }
                    }
                }
                // decrement sample rate
                Key::Minus => {
                    if sample_rate > 1. {
                        sample_rate -= base.pow(increment) as f64;
                        if sample_rate < 1. { sample_rate = 1. }
                    };
                }
                // pause/resume
                Key::P => {
                    if !paused {
                        paused = true;
                    } else {
                        paused = false;
                        ts = initialize_ts(start_ts);
                    }
                }
                // restart
                Key::R => {
                    curr_index = start_index;
                    last_index_flushed = start_index;
                    ts = initialize_ts(start_ts);
                    reloading_required = true;
                }
                // increment candle size
                Key::Up => {
                    candle_size += 1;
                    cs = initialize_buff_chart(&mut buf, &v[start_index..end_index], candle_size, start_index)?;
                    reloading_required = true;
                }
                // decrement candle size
                Key::Down => {
                    if candle_size > 1 {
                        candle_size -= 1;
                        cs = initialize_buff_chart(&mut buf, &v[start_index..end_index], candle_size, start_index)?;
                        reloading_required = true;
                    }
                }
                // go to next series
                Key::Right => {
                    v_index = (v_index + 1) % vec.len();
                    v = &vec[v_index].1;
                    v_name = *&vec[v_index].0;
                    start_index = 0;
                    end_index = v.len();
                    curr_index = 0;
                    last_index_flushed = 0;
                    ts = initialize_ts(start_ts);
                    cs = initialize_buff_chart(&mut buf, &v[start_index..end_index], candle_size, start_index)?;
                }
                // go to previous series
                Key::Left => {
                    if v_index == 0 { v_index = vec.len() - 1 } else { v_index -= 1 };
                    v = &vec[v_index].1;
                    v_name = *&vec[v_index].0;
                    start_index = 0;
                    end_index = v.len();
                    curr_index = 0;
                    last_index_flushed = 0;
                    ts = initialize_ts(start_ts);
                    cs = initialize_buff_chart(&mut buf, &v[start_index..end_index], candle_size, start_index)?;
                }
                // increment start index
                Key::Key1 => {
                    if start_index > 0 {
                        if start_index < base.pow(increment) { start_index = 0; } else { start_index -= base.pow(increment); }
                        cs = initialize_buff_chart(&mut buf, &v[start_index..end_index], candle_size, start_index)?;
                        reloading_required = true;
                    }
                }
                // decrement start index
                Key::Key2 => {
                    if start_index +2 < end_index
                        && (start_index + 1 < curr_index) {
                        start_index += base.pow(increment);
                        if start_index + 2 > end_index { start_index = end_index - 2; }
                        if start_index + 1 > curr_index { curr_index = start_index + 1; }
                        cs = initialize_buff_chart(&mut buf, &v[start_index..end_index], candle_size, start_index)?;
                        reloading_required = true;
                    }
                }
                // increment end index
                Key::Key0 => {
                    if end_index < v.len() {
                        end_index += base.pow(increment);
                        if end_index > v.len() { end_index = v.len(); }
                        cs = initialize_buff_chart(&mut buf, &v[start_index..end_index], candle_size, start_index)?;
                        reloading_required = true;
                    }
                }
                // decrement end index
                Key::Key9 => {
                    if end_index > start_index + 2 {
                        end_index -= base.pow(increment);
                        if end_index < start_index + 2 { end_index = start_index + 2; }
                        if end_index - 1 < curr_index { curr_index = end_index - 1; }
                        cs = initialize_buff_chart(&mut buf, &v[start_index..end_index], candle_size, start_index)?;
                        reloading_required = true;
                    }
                }
                _ => {
                    continue;
                }
            }
            window.set_title(&get_window_title(v_name, paused, candle_size, sample_rate, start_index, end_index));
        }
        window.update_with_buffer(buf.borrow(), W, H)?;
    }
    Ok(())
}