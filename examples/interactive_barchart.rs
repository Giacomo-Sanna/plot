use std::vec;
use plot_graph::{helpers, interactive_chart};

fn main() {
    let vec = vec![
        ("ABC/DEF", helpers::generate_data_series(100., 1000, -0.0985, 0.1)),
        ("UVW/XYZ", helpers::generate_data_series(100., 1000, -0.0985, 0.1))];
    interactive_chart::launch_gui_barchart(vec).expect("ERROR: Unable to launch GUI!");
}