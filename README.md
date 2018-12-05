# Advent of Code 2018 
My solutions to [Advent of Code](https://adventofcode.com/) written in Rust. 

I'm not focusing on anything special just to write some nice idiomatic Rust that is fast.

## How to run code
```bash
$ cargo test dayX::tests::test --release -- --nocapture
```

## How to run benchmarks
```bash
$ cargo bench
```

## Benchmarks

### Day 1
```bash
day1                    time:   [13.721 ms 13.927 ms 14.149 ms]                  
                        change: [-79.956% -79.617% -79.292%] (p = 0.00 < 0.05)

day2                    time:   [447.23 us 452.72 us 459.82 us]                 
                        change: [-0.5818% +1.8583% +4.5539%] (p = 0.16 > 0.05)

day3                    time:   [1.7930 ms 1.7965 ms 1.8001 ms]
                        change: [-8.6050% -7.9943% -7.4142%] (p = 0.00 < 0.05)
                        
day4                    time:   [688.17 us 689.10 us 690.10 us]                 
                        change: [-0.7025% -0.2898% +0.2260%] (p = 0.23 > 0.05)
                        
day5                    time:   [20.645 ms 20.654 ms 20.665 ms]                  
                        change: [-0.9743% -0.7658% -0.5080%] (p = 0.00 < 0.05)                        
```
