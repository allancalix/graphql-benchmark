# graphql-benchmark

```bash
cargo bench --bench benches
```

## Sample report
```
async_graphql_parser    time:   [7.5938 µs 7.6117 µs 7.6288 µs]
                        change: [-0.5916% -0.1561% +0.2545%] (p = 0.46 > 0.05)
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) low severe
  3 (3.00%) low mild
  1 (1.00%) high mild

apollo_graphql_parser   time:   [6.7100 µs 6.7274 µs 6.7436 µs]
                        change: [-1.8937% -1.5113% -1.1105%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) low mild
  1 (1.00%) high mild

apollo_fork_graphql_parser
                        time:   [4.7675 µs 4.7785 µs 4.7904 µs]
                        change: [-2.1926% -1.7055% -1.2349%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe

graphql_parser          time:   [5.6228 µs 5.6353 µs 5.6472 µs]
                        change: [-0.3519% +0.0893% +0.6192%] (p = 0.70 > 0.05)
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low severe
  6 (6.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe

async_graphql_parser #2 time:   [2.3517 ms 2.3572 ms 2.3625 ms]
                        change: [+1.3489% +1.6262% +1.9296%] (p = 0.00 < 0.05)
                        Performance has regressed.

apollo_graphql_parser #2
                        time:   [3.0019 ms 3.0110 ms 3.0203 ms]
                        change: [+1.9770% +2.3294% +2.6782%] (p = 0.00 < 0.05)
                        Performance has regressed.

apollo_fork_graphql_parser #2
                        time:   [1.9258 ms 1.9290 ms 1.9322 ms]
                        change: [+0.6851% +0.9991% +1.3283%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  2 (2.00%) high severe

graphql_parser #2       time:   [2.2415 ms 2.2494 ms 2.2584 ms]
                        change: [+0.1431% +0.5157% +0.9513%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high severe
```
