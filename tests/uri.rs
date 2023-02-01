use uri::authority::{Authority, Host, Port, UserInfo};
use uri::Parser;
use uri::uri::URI;


#[test]
fn decode1() {
    let a = "https://root:passwd@www.baidu.com:8081/path/to/my/value?arg1=1&arg2=hello&arg2=ddd#L18";
    let r = URI::decode(a);
    match r {
        Ok(u) => println!("{:?}", u),
        Err(e) => println!("{}", e),
    }
}

#[test]
fn decode2() {
    let a = "https://root:passwd@www.baidu.com:8081/path/to/my/value?arg1=1&arg2=hello&arg2=ddd#L18";
    let r = URI::decode(a);
    match r {
        Ok(u) => {
            println!("decode success");
            println!("{:?}", u);
            match u.encode() {
                Ok(eu) => {
                    println!("encode success");
                    println!("{}", eu);
                }
                Err(err) => {
                    println!("{}", err)
                }
            }
        }
        Err(e) => println!("{}", e),
    }
}

#[test]
fn decode3() {
    let a = "12345";
    match a {
        "12345" => { println!("1") }
        _ => { println!("match any", ) }
    };
}