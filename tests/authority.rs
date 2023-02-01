use std::net::Ipv4Addr;
use uri::authority::{Authority, Host, Port, UserInfo};
use uri::Parser;

#[test]
fn authority_encode() {
    let mut auth = Authority::new(Host::RegName(String::from("www.baidu.com")));
    // auth.set_host(Host::decode("www.baidu.com").unwrap());
    auth.set_port(Port::new(443));
    // auth.set_userinfo(UserInfo::new("root:passwd"));
    let r = auth.encode().unwrap();
    println!("{}", r)
}

#[test]
fn authority_decode() {
    let auth = Authority::decode("root:passwd@www.baidu.com:443").unwrap();
    println!("{:?}", auth)
}

#[test]
fn host_encode() {
    let host_ipl = Host::IPLiteral(String::from("11ff::3344"));
    match host_ipl.encode() {
        Ok(r) => println!("ip_literal host = {}", r),
        Err(e) => println!("ip_literal err = {}", e),
    }

    let host_v4addr = Host::IPv4Addr(Ipv4Addr::new(192, 168, 1, 1));
    match host_v4addr.encode() {
        Ok(v4addr) => println!("v4addr host = {}", v4addr),
        Err(e) => println!("v4addr err = {}", e),
    }

    let host_regname = Host::RegName(String::from("www.代澎源.com"));
    match host_regname.encode() {
        Ok(h) => println!("rename host = {}", h),
        Err(e) => println!("rename host err = {}", e),
    }
}

#[test]
fn host_decode() {
    let ipl = "[fe03::0001]";
    match Host::decode(ipl) {
        Ok(host) => println!("ip_literal host = {:?}", host),
        Err(e) => println!("ip_literal err = {:?}", e),
    }

    let v4addr = "192.168.1.2";
    match Host::decode(v4addr) {
        Ok(host) => println!("v4addr host = {:?}", host),
        Err(e) => println!("v4addr err = {:?}", e),
    }

    let regname = "www.%E4%BB%A3%E6%BE%8E%E6%BA%90.com";
    match Host::decode(regname) {
        Ok(host) => println!("regname host = {:?}", host),
        Err(e) => println!("regname err = {:?}", e),
    }
}

#[test]
fn userinfo_encode() {
    let u = UserInfo::new("root:ddd");
    let r = u.encode().unwrap();
    println!("{}", r)
}

#[test]
fn userinfo_decode() {
    let u = "root:yyy";
    let r = UserInfo::decode(u).unwrap();
    println!("{:?}", r)
}

#[test]
fn port_encode() {
    let u = Port::new(443);
    let r = u.encode().unwrap();
    println!("{}", r)
}

#[test]
fn port_decode() {
    let p = "443";
    let r = Port::decode(p).unwrap();
    println!("{:?}", r)
}

#[test]
fn pct_oper() {
    let a = "代";
    println!("pct_encode = {}", pct_encode(a));

    let b = "%E4%BB%A3%E6%BE%8E%E6%BA%90";
    println!("pct_decode = {}", pct_decode(b));
}

fn pct_encode(s: &str) -> String {
    let mut buf = String::new();
    let mut iter = s.as_bytes().iter();
    while let Some(item) = iter.next() {
        buf.push_str(format!("%{:X}", *item).as_str())
    }
    buf
}

fn pct_decode(s: &str) -> String {
    let mut buf = Vec::<u8>::new();
    let mut iter = s.as_bytes().iter();
    while let Some(item) = iter.next() {
        if *item == b'%' {
            let pb1 = iter.next().unwrap();
            let pb2 = iter.next().unwrap();
            let subs = String::from_utf8([*pb1, *pb2].to_vec()).unwrap();
            let sub = u8::from_str_radix(subs.as_str(), 16).unwrap();
            buf.push(sub)
        } else {
            buf.push(*item)
        }
    }
    String::from_utf8(buf).unwrap()
}