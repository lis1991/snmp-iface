# snmp-iface

SNMP network interface table viewer. Pure Rust, no system dependencies.

## Usage

```
snmp-iface [OPTIONS]

Options:
  --host <HOST>            Target host [default: 127.0.0.1]
  --community <COMMUNITY>  SNMP community string [default: public]
  --port <PORT>            SNMP UDP port [default: 161]
  -h, --help               Print help
  -V, --version            Print version
```

### Examples

```bash
# localhost
./snmp-iface

# remote host
./snmp-iface --host 172.16.0.167

# full options
./snmp-iface --host 172.16.0.167 --community public --port 161
```

## Build

```bash
cargo build --release
```

Binary will be at `target/release/snmp-iface`.

## Output

```
SNMP Interface Table
  host      : 127.0.0.1:161
  community : public

+------+------------+-------------+-------+--------+--------+---------------------+-------------+-------------+----------------+----------+----------+
| Idx  | Port       | Speed       | MTU   | Admin  | Oper   | MAC                 | In traffic  | Out traffic | In pkts        | In err   | Out err  |
+------+------------+-------------+-------+--------+--------+---------------------+-------------+-------------+----------------+----------+----------+
| 1    | lo         | 10 Mbit/s   | 65536 | [UP  ] | [UP  ] | ---                 | 1.4 GB      | 1.4 GB      | 97,532,354     | 0        | 0        |
| 2    | eth1       | 100 Mbit/s  | 1500  | [UP  ] | [UP  ] | 9c:d3:32:00:4c:63   | 1.1 GB      | 221.4 MB    | 75,964,247     | 0        | 0        |
+------+------------+-------------+-------+--------+--------+---------------------+-------------+-------------+----------------+----------+----------+
  Total interfaces: 2
```
