
# Install libpg
Diesel needs the native postgress library:
`sudo apt install libpq-dev`

# diesel CLI
Diesel CLI runs migrations and creates the schema file

## Install
`cargo install diesel_cli --no-default-features --features postgres`

## Setup schema
`diesel setup`

## Run migrations
`diesel migration run`

# Benchmark
`$ autocannon http://localhost:3080/test`

```
┌─────────┬──────┬──────┬───────┬──────┬─────────┬─────────┬──────┐
│ Stat    │ 2.5% │ 50%  │ 97.5% │ 99%  │ Avg     │ Stdev   │ Max  │
├─────────┼──────┼──────┼───────┼──────┼─────────┼─────────┼──────┤
│ Latency │ 0 ms │ 0 ms │ 0 ms  │ 0 ms │ 0.01 ms │ 0.06 ms │ 8 ms │
└─────────┴──────┴──────┴───────┴──────┴─────────┴─────────┴──────┘
┌───────────┬────────┬────────┬─────────┬─────────┬─────────┬────────┬────────┐
│ Stat      │ 1%     │ 2.5%   │ 50%     │ 97.5%   │ Avg     │ Stdev  │ Min    │
├───────────┼────────┼────────┼─────────┼─────────┼─────────┼────────┼────────┤
│ Req/Sec   │ 39071  │ 39071  │ 45823   │ 46079   │ 45004.8 │ 2025.2 │ 39060  │
├───────────┼────────┼────────┼─────────┼─────────┼─────────┼────────┼────────┤
│ Bytes/Sec │ 3.2 MB │ 3.2 MB │ 3.76 MB │ 3.78 MB │ 3.69 MB │ 166 kB │ 3.2 MB │
└───────────┴────────┴────────┴─────────┴─────────┴─────────┴────────┴────────┘
```
