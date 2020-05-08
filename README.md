# sentinel-rust

## Usage

For development purposes:

```
cargo run
```

Building a production release with:

```
cargo build -j <NUM_CPU_THREADS> --release

# First, configure `./conf/production/config.yml`
$ RUN_MODE=production ./sentinel
```

## Tests

```
cargo test
```

## Docs

```
cargo doc
```

## Extras

Here are some initial build stats:

```
Threadripper (v1) 1950X 16C/32T in XCP-ng virtualised as 20T
Ubuntu 18.04.4 LTS 4.15.0-99-generic
stable-x86_64-unknown-linux-gnu (default)
rustc 1.43.1 (8d69840ab 2020-05-04)

cargo build -j 20 --release
Finished release [optimized] target(s) in 48.79s                                                            
real    0m48.810s                                         
user    10m56.324s                                        
sys     0m13.545s 


The iMac "Core i7" 4.0 27-Inch (Late 2014/Retina 5K)
w/ 22 nm "Haswell" Quad Core 4.0 GHz Intel "Core i7" (4790K)
MacOS 10.14
stable-x86_64-apple-darwin (default)
rustc 1.43.1 (8d69840ab 2020-05-04)

cargo build -j 8 --release
Finished release [optimized] target(s) in 2m 58s

2:59.00 total
1163.24s user
35.09s system
669% cpu
```