use clap::Parser;
use snmp2::{SyncSession, Value};
use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::time::Duration;

/// SNMP network interface table viewer
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Target host (IP or hostname)
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// SNMP community string
    #[arg(long, default_value = "public")]
    community: String,

    /// SNMP UDP port
    #[arg(long, default_value_t = 161)]
    port: u16,
}

const OID_DESCR:   &[u32] = &[1,3,6,1,2,1,2,2,1,2];
const OID_SPEED:   &[u32] = &[1,3,6,1,2,1,2,2,1,5];
const OID_MTU:     &[u32] = &[1,3,6,1,2,1,2,2,1,4];
const OID_MAC:     &[u32] = &[1,3,6,1,2,1,2,2,1,6];
const OID_ADMIN:   &[u32] = &[1,3,6,1,2,1,2,2,1,7];
const OID_OPER:    &[u32] = &[1,3,6,1,2,1,2,2,1,8];
const OID_IN_OCT:  &[u32] = &[1,3,6,1,2,1,2,2,1,10];
const OID_OUT_OCT: &[u32] = &[1,3,6,1,2,1,2,2,1,16];
const OID_IN_PKT:  &[u32] = &[1,3,6,1,2,1,2,2,1,11];
const OID_IN_ERR:  &[u32] = &[1,3,6,1,2,1,2,2,1,14];
const OID_OUT_ERR: &[u32] = &[1,3,6,1,2,1,2,2,1,20];

fn fmt_bytes(b: u64) -> String {
    if b >= 1_073_741_824 { format!("{:.1} GB", b as f64 / 1_073_741_824.0) }
    else if b >= 1_048_576 { format!("{:.1} MB", b as f64 / 1_048_576.0) }
    else if b >= 1_024     { format!("{:.1} KB", b as f64 / 1_024.0) }
    else                   { format!("{} B", b) }
}

fn fmt_speed(bps: u32) -> String {
    if bps >= 1_000_000_000 { format!("{} Gbit/s", bps / 1_000_000_000) }
    else if bps >= 1_000_000 { format!("{} Mbit/s", bps / 1_000_000) }
    else if bps >= 1_000     { format!("{} Kbit/s", bps / 1_000) }
    else if bps == 0         { "---".into() }
    else                     { format!("{} bit/s", bps) }
}

fn fmt_status(v: u32) -> &'static str {
    match v {
        1 => "[UP  ]",
        2 => "[DOWN]",
        3 => "[TEST]",
        _ => "[????]",
    }
}

fn fmt_num(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 { result.push(','); }
        result.push(c);
    }
    result.chars().rev().collect()
}

fn snmp_walk(sess: &mut SyncSession, base_oid: &[u32]) -> BTreeMap<u32, Value<'static>> {
    let mut map = BTreeMap::new();
    let mut current: Vec<u32> = base_oid.to_vec();

    loop {
        // snmp2 v0.3.x: OID is passed as &[u32] directly, no ObjectIdentifier wrapper
        match sess.getnext(&[current.as_slice()]) {
            Ok(response) => {
                if let Some((next_oid, val)) = response.varbinds.next() {
                    // snmp2 v0.3.x: next_oid.iter() returns Option<impl Iterator<Item=u64>>
                    // unwrap the Option, then map u64 -> u32
                    let next_arcs: Vec<u32> = match next_oid.iter() {
                        Some(it) => it.map(|v| v as u32).collect(),
                        None => break,
                    };
                    if next_arcs.len() <= base_oid.len()
                        || &next_arcs[..base_oid.len()] != base_oid
                    {
                        break;
                    }
                    let idx: u32 = *next_arcs.last().unwrap();
                    let owned: Value<'static> = match val {
                        Value::Integer(i)      => Value::Integer(i),
                        Value::Counter32(i)    => Value::Counter32(i),
                        Value::Unsigned32(i)   => Value::Unsigned32(i),
                        Value::Timeticks(i)    => Value::Timeticks(i),
                        Value::OctetString(s)  => {
                            let v: Vec<u8> = s.to_vec();
                            Value::OctetString(Box::leak(v.into_boxed_slice()))
                        }
                        _ => Value::Integer(0),
                    };
                    map.insert(idx, owned);
                    current = next_arcs;
                } else {
                    break;
                }
            }
            Err(_) => break,
        }
    }
    map
}

fn val_u32(map: &BTreeMap<u32, Value>, idx: u32) -> u32 {
    match map.get(&idx) {
        Some(Value::Integer(v))    => *v as u32,
        Some(Value::Counter32(v))  => *v,
        Some(Value::Unsigned32(v)) => *v,
        _ => 0,
    }
}

fn val_u64(map: &BTreeMap<u32, Value>, idx: u32) -> u64 {
    val_u32(map, idx) as u64
}

fn val_str(map: &BTreeMap<u32, Value>, idx: u32) -> String {
    match map.get(&idx) {
        Some(Value::OctetString(s)) => String::from_utf8_lossy(s).trim().to_string(),
        _ => "?".into(),
    }
}

fn val_mac(map: &BTreeMap<u32, Value>, idx: u32) -> String {
    match map.get(&idx) {
        Some(Value::OctetString(s)) if s.len() == 6 => {
            format!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                s[0], s[1], s[2], s[3], s[4], s[5])
        }
        _ => "---".into(),
    }
}

fn main() {
    let args = Args::parse();

    let addr: SocketAddr = format!("{}:{}", args.host, args.port)
        .parse()
        .unwrap_or_else(|_| { eprintln!("ERROR: invalid host/port"); std::process::exit(1); });

    let community = args.community.as_bytes().to_vec();

    // snmp2 v0.3.x SyncSession::new(addr, community: &[u8], timeout, retries)
    // community must be &[u8], addr must be SocketAddr
    let mut sess = SyncSession::new(addr, &community, Some(Duration::from_secs(3)), 2)
        .unwrap_or_else(|e| { eprintln!("ERROR: cannot create session: {e}"); std::process::exit(1); });

    let descr   = snmp_walk(&mut sess, OID_DESCR);
    let speed   = snmp_walk(&mut sess, OID_SPEED);
    let mtu     = snmp_walk(&mut sess, OID_MTU);
    let mac     = snmp_walk(&mut sess, OID_MAC);
    let admin   = snmp_walk(&mut sess, OID_ADMIN);
    let oper    = snmp_walk(&mut sess, OID_OPER);
    let in_oct  = snmp_walk(&mut sess, OID_IN_OCT);
    let out_oct = snmp_walk(&mut sess, OID_OUT_OCT);
    let in_pkt  = snmp_walk(&mut sess, OID_IN_PKT);
    let in_err  = snmp_walk(&mut sess, OID_IN_ERR);
    let out_err = snmp_walk(&mut sess, OID_OUT_ERR);

    let indexes: Vec<u32> = {
        let mut v: Vec<u32> = descr.keys().cloned().collect();
        v.sort();
        v
    };

    if indexes.is_empty() {
        eprintln!("\nERROR: no interfaces found.");
        eprintln!("  host      : {}:{}", args.host, args.port);
        eprintln!("  community : {}", args.community);
        std::process::exit(1);
    }

    let sep = "+------+------------+-------------+-------+--------+--------+---------------------+-------------+-------------+----------------+----------+----------+";
    let hdr = "| Idx  | Port       | Speed       | MTU   | Admin  | Oper   | MAC                 | In traffic  | Out traffic | In pkts        | In err   | Out err  |";

    println!("\nSNMP Interface Table");
    println!("  host      : {}:{}", args.host, args.port);
    println!("  community : {}", args.community);
    println!();
    println!("{sep}");
    println!("{hdr}");
    println!("{sep}");

    for idx in &indexes {
        let i = *idx;
        println!(
            "| {:<4} | {:<10} | {:<11} | {:<5} | {:<6} | {:<6} | {:<19} | {:<11} | {:<11} | {:<14} | {:<8} | {:<8} |",
            i,
            val_str(&descr, i),
            fmt_speed(val_u32(&speed, i)),
            val_u32(&mtu, i),
            fmt_status(val_u32(&admin, i)),
            fmt_status(val_u32(&oper, i)),
            val_mac(&mac, i),
            fmt_bytes(val_u64(&in_oct, i)),
            fmt_bytes(val_u64(&out_oct, i)),
            fmt_num(val_u64(&in_pkt, i)),
            val_u64(&in_err, i),
            val_u64(&out_err, i),
        );
    }

    println!("{sep}");
    println!("  Total interfaces: {}\n", indexes.len());
}
