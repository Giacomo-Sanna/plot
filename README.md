# Charts

## Grafico a candele interattivo
![gif architettura](./images/candles.gif)
Eseguibile tramite il seguente comando
```rust
cargo run --example interactive_candles 
```
### Instructions:
- ←/→ = Previous/next series
- ↑/↓ = Adjust candle size
- +/- = Adjust sample rate
- 1/2 = Adjust start index
- 9/0 = Adjust end index
- P = Start/Stop
- R = Restart
- \<Esc> = Exit

## Base plot
Per eseguire tutti i grafici
```rust
cargo test
```


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

## Candlestick chart
```rust
use candles::plot;
candles::plot(vec![..], <candle_size>, "file_name", "caption");
```


## Bar chart (liquidity chart)
```rust
use liquidity_chart::plot;
liquidity_plot::plot(vec![vec![..], ..], vec![<market_names>], "file_name", "caption");
```
