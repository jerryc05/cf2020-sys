# Cloudflare 2020 System Engineering Assignment

This repo is created by Ziyan "Jerry" Chen.

## Usage

```
cf2020-sys 0.1.0
Ziyan "Jerry" Chen <jerryc443@gmail.com>
Cloudflare 2020 System Engineering Assignment

USAGE:
    ./cf2020-sys [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Enable verbose output.

OPTIONS:
    -p, --profile <n>    Number of requests.
    -u, --url <url>      The URL to test. Must be one of [http(s)://*.example.com/*] or [*.example.com/*]

```

## Profile findings

I ran my profile function on both [GitHub](https://github.com/) and 
[My CF Workers Site](https://cf2020.jerryc05.workers.dev), and I found 
that requests sent to My CF Workers Site were much faster than that to
GitHub (`208ms` vs `453ms` on avg). 

However, I also noticed that the 
data sent from GitHub was much larger (`221k bytes` vs `1k bytes`) than 
that from My CF Workers Site, and this probably explained the profiling results.