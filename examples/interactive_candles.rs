use std::borrow::{Borrow, BorrowMut};
use plotters::prelude::*;
use std::error::Error;
use minifb::{Key, /*KeyRepeat,*/ Window, WindowOptions};
use plotters_bitmap::bitmap_pixel::BGRXPixel;
use plotters_bitmap::BitMapBackend;
use std::time::SystemTime;
use plotters::chart::ChartState;
use plotters::coord::types::{RangedCoordf32, RangedCoordusize};
use plot_graph;
use plot_graph::helpers::BufferWrapper;

const W: usize = 800;
const H: usize = 600;
const MARGIN: u32 = 10;
const LABEL: u32 = 30;

const SAMPLE_RATE: f64 = 60.0;
// const FRAME_RATE: f64 = 60.;

fn initialize_buff_chart(buf: & mut BufferWrapper, v: &[f32], candle_size: usize) ->  Result<ChartState<Cartesian2d<RangedCoordusize, RangedCoordf32>>, Box<dyn Error>> {
    let el_max = plot_graph::helpers::f32_max(&v);
    let el_min = plot_graph::helpers::f32_min(&v);

    let candles_data = plot_graph::candles::parse_data(&v, candle_size, None);

    // Get date range
    let (start_date, end_date) = (
        // 0,
        // n_elements + 1,
        candles_data[0].0 - 1,
        candles_data[candles_data.len() - 1].0 + 1,
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
            //.label_style(("sans-serif", 15).into_font().color(&GREEN))
            //.axis_style(&GREEN)
            .light_line_style(&WHITE)
            .draw()?;

        let cs = chart.into_chart_state();
        root.present()?;
        cs
    };
    Ok(cs)
}

fn draw_buff_chart(buf: & mut BufferWrapper, v: &[f32], candle_size: usize, curr_index: usize, cs: &ChartState<Cartesian2d<RangedCoordusize, RangedCoordf32>>) -> Result<(), Box<dyn Error>> {
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
            //.bold_line_style(&GREEN.mix(0.2))
            //.light_line_style(&TRANSPARENT)
            .light_line_style(&WHITE)
            .draw()?;

        // data.push(candles_left.pop().unwrap());

        // let data = vec![candles_left.pop().unwrap(); 1];

        let data = plot_graph::candles::parse_data(&v[..curr_index], candle_size, None);
        // println!("\ncurr_index: {}, data: {:?}\n", curr_index, data);
        let n_elements = plot_graph::candles::parse_data(&v, candle_size, None).len();

        chart
            .draw_series(data.iter().map(|x| {
                CandleStick::new(
                    x.0,
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

pub fn main() -> Result<(), Box<dyn Error>> {
    let v = plot_graph::helpers::generate_data_series(100., 1000, -0.0985, 0.1);
    let candle_size = 10;
    // println!("v: {:?}", v);
    // println!("v len: {:?}", v.len());

    //let candles_data = plot_graph::candles::parse_data(&v, candle_size);

    let mut buf = BufferWrapper::new(W, H);

    // let mut fx: f64 = 1.0;
    // let mut fy: f64 = 1.1;
    // let mut xphase: f64 = 0.0;
    // let mut yphase: f64 = 0.1;

    let mut window = Window::new(
        //&get_window_title(fx, fy, yphase - xphase),
        "Window Title",
        W,
        H,
        WindowOptions::default(),
    )?;

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let cs = initialize_buff_chart(& mut buf, &v, candle_size)?;

    let mut curr_index = 0;
    let mut last_index_flushed = 0;

    // let mut data = VecDeque::new();
    let start_ts = SystemTime::now();
    let mut ts = SystemTime::now()
        .duration_since(start_ts)
        .unwrap()
        .as_secs_f64();
    // let mut last_flushed = 0.0;
    // let mut last_data_change = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let epoch = SystemTime::now()
            .duration_since(start_ts)
            .unwrap()
            .as_secs_f64();

        while ts < epoch && curr_index < v.len() {
            ts += 1.0 / SAMPLE_RATE;
            curr_index += 1;
        }

        // if curr_index < v.len() {
        //     if epoch - last_data_change < 0.1 / SAMPLE_RATE {
        //         std::thread::sleep(std::time::Duration::from_secs_f64(epoch - last_data_change));
        //         continue;
        //     }
        //     println!("epoch: {}, curr_index: {}, v.len(): {}", epoch, curr_index, v.len());
        //     curr_index += 1;
        //     println!("epoch: {}, curr_index: {}, v.len(): {}", epoch, curr_index, v.len());
        //     last_data_change = epoch;
        // }

        // if let Some((ts, _, _)) = data.back() {
        //     if epoch - ts < 1.0 / SAMPLE_RATE {
        //         std::thread::sleep(std::time::Duration::from_secs_f64(epoch - ts));
        //         continue;
        //     }
        //     let mut ts = *ts;
        //     while ts < epoch {
        //         ts += 1.0 / SAMPLE_RATE;
        //         let phase_x: f64 = 2.0 * ts * std::f64::consts::PI * fx + xphase;
        //         let phase_y: f64 = 2.0 * ts * std::f64::consts::PI * fy + yphase;
        //         data.push_back((ts, phase_x.sin(), phase_y.sin()));
        //     }
        // }
        //
        // let phase_x = 2.0 * epoch * std::f64::consts::PI * fx + xphase;
        // let phase_y = 2.0 * epoch * std::f64::consts::PI * fy + yphase;
        // data.push_back((epoch, phase_x.sin(), phase_y.sin()));

        // if epoch - last_flushed > 1.0 / FRAME_RATE
        {
            //println!("curr_index: {}, v.len() {}", curr_index, v.len());
            if last_index_flushed < curr_index {
                last_index_flushed = curr_index;

                draw_buff_chart(&mut buf, &v, candle_size, curr_index, &cs)?;

                // let keys = window.get_keys_pressed(KeyRepeat::Yes);
                // for key in keys {
                //     let old_fx = fx;
                //     let old_fy = fy;
                //     match key {
                //         Key::Equal => {
                //             fy += 0.1;
                //         }
                //         Key::Minus => {
                //             fy -= 0.1;
                //         }
                //         Key::Key0 => {
                //             fx += 0.1;
                //         }
                //         Key::Key9 => {
                //             fx -= 0.1;
                //         }
                //         _ => {
                //             continue;
                //         }
                //     }
                //     xphase += 2.0 * epoch * std::f64::consts::PI * (old_fx - fx);
                //     yphase += 2.0 * epoch * std::f64::consts::PI * (old_fy - fy);
                //     window.set_title(&get_window_title(fx, fy, yphase - xphase));
                // }
            }
            window.update_with_buffer(buf.borrow(), W, H)?;
            // last_flushed = epoch;
        }
    }
    Ok(())
}