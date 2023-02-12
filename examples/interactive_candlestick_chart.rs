use std::vec;
use plot::{helpers, interactive_chart};

fn main() {
    let vec = vec![
        ("ABC/DEF", helpers::generate_data_series(100., 1000, -0.0985, 0.1)),
        ("UVW/XYZ", helpers::generate_data_series(100., 1000, -0.0985, 0.1))];
    println!("{}", interactive_chart::get_instructions(interactive_chart::ChartType::Candlestick));
    interactive_chart::launch_gui_candlestick(vec, Some(10)).expect("ERROR: Unable to launch GUI!");
}