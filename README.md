# Advent of Code 2018 
My solutions to [Advent of Code](https://adventofcode.com/) written in Rust. 

I'm not focusing special just to write some nice idiomatic Rust that is fast.

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

```
