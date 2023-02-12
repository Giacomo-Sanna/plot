use std::borrow::{Borrow};
use plotters::prelude::*;
use std::error::Error;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::time::SystemTime;
use plotters::chart::ChartState;
use plotters::coord::types::{RangedCoordf32, RangedCoordi32};
use crate::helpers;
use crate::interactive_candlestick_chart;
use crate::interactive_bar_chart;
use crate::helpers::interactive_chart::*;

pub enum ChartType {
    Bar,
    Candlestick,
}

pub enum ChartStateWrapper {
    Bar(ChartState<Cartesian2d<RangedCoordf32, RangedCoordf32>>),
    Candlestick(ChartState<Cartesian2d<RangedCoordi32, RangedCoordf32>>),
}

fn initialize_buff_chart(chart_type: &ChartType, buf: &mut helpers::BufferWrapper, v: &[f32], candle_size: usize, start_index: usize)
    -> Result<ChartStateWrapper, Box<dyn Error>> {
    match chart_type {
        ChartType::Candlestick => {
            let rtn = interactive_candlestick_chart::initialize_buff_chart(buf, v, candle_size, start_index)?;
            Ok(ChartStateWrapper::Candlestick(rtn))
        }
        ChartType::Bar => {
            let rtn = interactive_bar_chart::initialize_buff_chart(buf, v, start_index)?;
            Ok(ChartStateWrapper::Bar(rtn))
        }
    }
}

fn draw_buff_chart(buf: &mut helpers::BufferWrapper, v: &[f32], candle_size: usize, curr_index: usize, start_index: usize,
                   cs: &ChartStateWrapper) -> Result<(), Box<dyn Error>> {
    match cs {
        ChartStateWrapper::Candlestick(cs) => {
            interactive_candlestick_chart::draw_buff_chart(buf, v, candle_size, curr_index, start_index, cs)?;
            Ok(())
        }
        ChartStateWrapper::Bar(cs) => {
            interactive_bar_chart::draw_buff_chart(buf, v, curr_index, start_index, cs, None)?;
            Ok(())
        }
    }
}

fn get_window_title(chart_type: &ChartType, v_name: &str, paused: bool, candle_size: usize, sr: f64, start_index: usize, end_index: usize) -> String {
    match chart_type {
        ChartType::Candlestick => { interactive_candlestick_chart::get_window_title(v_name, paused, candle_size, sr, start_index, end_index)}
        ChartType::Bar => { interactive_bar_chart::get_window_title(v_name, paused, sr, start_index, end_index)}
    }
}

fn initialize_ts(start_ts: SystemTime) -> f64 {
    SystemTime::now()
        .duration_since(start_ts)
        .unwrap()
        .as_secs_f64()
}

pub fn launch_gui_barchart(vec: Vec<(&str, Vec<f32>)>) -> Result<(), Box<dyn Error>> {
    launch_gui(ChartType::Bar, vec, None)
}

pub fn launch_gui_candlestick(vec: Vec<(&str, Vec<f32>)>, candle_size: Option<usize>) -> Result<(), Box<dyn Error>> {
    launch_gui(ChartType::Candlestick, vec, candle_size)
}

pub fn get_instructions(chart_type: ChartType) -> String {
    match &chart_type {
        ChartType::Candlestick => { interactive_candlestick_chart::INSTRUCTIONS.to_string()}
        ChartType::Bar => { interactive_bar_chart::INSTRUCTIONS.to_string()}
    }
}

fn launch_gui(chart_type: ChartType, vec: Vec<(&str, Vec<f32>)>, candle_size: Option<usize>) -> Result<(), Box<dyn Error>> {
    let mut v_index = 0;
    let mut v = &vec[v_index].1;
    let mut v_name = *&vec[v_index].0;

    let mut buf = helpers::BufferWrapper::new(W, H);
    let mut candle_size = match candle_size {
        Some(x) => x,
        None => 10,
    };
    let mut paused = false;
    let mut sample_rate = SAMPLE_RATE;
    let mut start_index = 0;
    let mut end_index = v.len();

    let mut window = Window::new(
        &get_window_title(&chart_type, v_name, paused, candle_size, sample_rate, start_index, end_index),
        W,
        H,
        WindowOptions::default(),
    )?;

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut cs = initialize_buff_chart(&chart_type, &mut buf, &v, candle_size, start_index)?;

    let mut chart_reloading_required = false;
    let mut chart_initialization_required = false;
    let mut curr_index = 0;
    let mut last_index_flushed = 0;

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
            || chart_reloading_required)
            && curr_index > start_index {

            if chart_initialization_required {
                cs = initialize_buff_chart(&chart_type, &mut buf, &v[start_index..end_index], candle_size, start_index)?;
                chart_initialization_required = false;
            }

            last_index_flushed = curr_index;
            chart_reloading_required = false;
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
                    chart_reloading_required = true;
                }
                // increment candle size
                Key::Up => {
                    match &chart_type {
                        ChartType::Candlestick => {
                            candle_size += 1;
                            chart_initialization_required = true;
                            chart_reloading_required = true;
                        }
                        _ => { }
                    }
                }
                // decrement candle size
                Key::Down => {
                    match &chart_type {
                        ChartType::Candlestick => {
                            if candle_size > 1 {
                                candle_size -= 1;
                                chart_initialization_required = true;
                                chart_reloading_required = true;
                            }
                        }
                        _ => { }
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
                    chart_initialization_required = true;
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
                    chart_initialization_required = true;
                }
                // decrement start index
                Key::Key1 => {
                    if start_index > 0 {
                        if start_index < base.pow(increment) { start_index = 0; } else { start_index -= base.pow(increment); }
                        chart_initialization_required = true;
                        chart_reloading_required = true;
                    }
                }
                // increment start index
                Key::Key2 => {
                    if start_index +2 < end_index
                        // && (start_index + 1 < curr_index)
                    {
                        start_index += base.pow(increment);
                        if start_index + 2 > end_index { start_index = end_index - 2; }
                        if start_index + 1 > curr_index { curr_index = start_index + 1; }
                        chart_initialization_required = true;
                        chart_reloading_required = true;
                    }
                }
                // increment end index
                Key::Key0 => {
                    if end_index < v.len() {
                        end_index += base.pow(increment);
                        if end_index > v.len() { end_index = v.len(); }
                        chart_initialization_required = true;
                        chart_reloading_required = true;
                    }
                }
                // decrement end index
                Key::Key9 => {
                    if end_index > start_index + 2 {
                        if end_index < base.pow(increment) + start_index + 2 { end_index = start_index + 2; }
                        else { end_index -= base.pow(increment); }
                        if end_index - 1 < curr_index { curr_index = end_index - 1; }
                        chart_initialization_required = true;
                        chart_reloading_required = true;
                    }
                }
                _ => {
                    continue;
                }
            }
            window.set_title(&get_window_title(&chart_type, v_name, paused, candle_size, sample_rate, start_index, end_index));
        }
        window.update_with_buffer(buf.borrow(), W, H)?;
    }
    Ok(())
}