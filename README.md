# graphql-benchmark

```bash
cargo bench --bench benches
```

## Sample report
```
async_graphql_parser    time:   [7.7119 µs 7.7236 µs 7.7371 µs]
                        change: [-0.3155% +0.3756% +1.1266%] (p = 0.32 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) high mild
  8 (8.00%) high severe

apollo_graphql_parser   time:   [5.1162 µs 5.1346 µs 5.1519 µs]
                        change: [-1.1433% -0.7274% -0.3439%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

apollo_fork_graphql_parser
                        time:   [4.2137 µs 4.2297 µs 4.2476 µs]
                        change: [+0.1201% +0.6564% +1.3334%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe

bluejay_parser          time:   [1.3238 µs 1.3272 µs 1.3308 µs]
                        change: [-1.0971% -0.6939% -0.3049%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

graphql_parser          time:   [4.4966 µs 4.5078 µs 4.5218 µs]
                        change: [-5.8770% -3.4147% -1.8471%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) high mild
  6 (6.00%) high severe

async_graphql_parser #2 time:   [2.4506 ms 2.4988 ms 2.5586 ms]
                        change: [-4.2259% -1.7722% +0.8463%] (p = 0.19 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

apollo_graphql_parser #2
                        time:   [2.0481 ms 2.0560 ms 2.0646 ms]
                        change: [-2.7317% -1.9902% -1.3125%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

Benchmarking apollo_fork_graphql_parser #2: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 8.8s, enable flat sampling, or reduce sample count to 50.
apollo_fork_graphql_parser #2
                        time:   [1.8269 ms 1.8589 ms 1.8947 ms]
                        change: [+2.2312% +3.5430% +5.2907%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

bluejay_parser #2       time:   [565.76 µs 570.88 µs 575.81 µs]
                        change: [-0.7201% +0.1504% +1.0830%] (p = 0.74 > 0.05)
                        No change in performance detected.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe

Benchmarking graphql_parser #2: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.7s, enable flat sampling, or reduce sample count to 50.
graphql_parser #2       time:   [1.8857 ms 1.8957 ms 1.9065 ms]
                        change: [+0.0580% +0.7540% +1.4148%] (p = 0.03 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
```
