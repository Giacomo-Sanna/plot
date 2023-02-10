use std::vec;
use plot_graph::{helpers, interactive_candles};

fn main() {
    let vec = vec![
        ("ABC/DEF", helpers::generate_data_series(100., 1000, -0.0985, 0.1)),
        ("UVW/XYZ", helpers::generate_data_series(100., 1000, -0.0985, 0.1))];
    interactive_candles::launch_gui(vec, 10).expect("ERROR: Unable to launch GUI!");
}