## Simple Hosts Based DNS Blocking

Simple utility to manage DNS block lists on a computer by computer basis.

I love the PiHole and pfBlocker-ng projects, but move around outside of my home network enough that I dont have their full benifit most of the time.

This tool is currently way overkill for what it does, but as functionality gets fleshed out, the benifit will (hopefully) become aparant.

## Usage

### Sample config file

Below is a sample config file for the tool.
NOTE: Although all fields are required, currently only the `url` field is functional

```
[[blocklist]]
name = "StevenBlack"
url = "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts"
update = "7 days"
```

### Basic

Current usage is limited to `basic`

```
rhole 0.1.0
drew <drew@parker.systems>
Declare submodule argument types for matching

USAGE:
    rhole <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    basic      simple download and rewrite of hosts
    db-mgmt    Declare submodule argument types for matching
    help       Prints this message or the help of the given subcommand(s)
```

```
❯ rhole basic --config sample_config.toml -o fake_hosts
Config Contaings 1 Sources
Processing StevenBlack
Processed 177899 entries from StevenBlack
❯ head -n 20 fake_hosts
127.0.0.1       localhost
127.0.1.1       <redacted>

# ___rhole___
0.0.0.0 info-miasto.click
0.0.0.0 d3v2jn7zu5rnnq.cloudfront.net
0.0.0.0 dpd-statusdirect.xyz
0.0.0.0 3325604.notifysrv.com
0.0.0.0 surfsecured.net
0.0.0.0 www.goonline-bnpparibas-pl.xyz
0.0.0.0 nisaaweb.com.pl
0.0.0.0 tedioustooth.com
0.0.0.0 2405.content.swrve.com
0.0.0.0 recoadministratorsect.europeloadbalancer.web.machinevision.net.zooplus.it
0.0.0.0 www.dekoratos.pl
0.0.0.0 info.spirol.com
0.0.0.0 lt3.hit.stat24.com
0.0.0.0 www.rndnkpril1.pl-invest.site
0.0.0.0 www.vinted-pl-gj32d.nhadat247.xyz
0.0.0.0 www.vinted-pl-gj32d.fulidh.live
```
