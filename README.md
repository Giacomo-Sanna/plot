# Come mostrare grafico
Prerequisito: creare la cartella `plots-output` sotto `root`

## Base plot

### una serie
```rust
use plot_graph::base_plot;
base_plot::plot(vec![1., 3.], "file_name", "caption").expect("");
```

### più serie
```rust
use plot_graph::base_plot;
base_plot::plot_multiple_series(vec![vec![..], vec![..], ..], "file_name", "caption").expect("");
```