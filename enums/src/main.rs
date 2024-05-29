enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Ip_Addr {
    V4(String),
    V6(String),
}

enum Ip_Addr2 {
    V4(u8,u8,u8,u8),
    V6(String),
}

struct Ipv6Addr{
    // --snip--
}

struct Ipv4Addr{
    // --snip--
}
enum Ip_AddrStr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        //mothod body would be defined here
    }
}

struct QuitMessage; //unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); //tubple struct
struct ChangeColorMessage(i32,i32, i32); //tuple struct

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home =Ip_Addr::V4(String::from("127.0.1"));
    let loopback = Ip_Addr::V6(String::from("::1"));

    let home = Ip_Addr2::V4(127,0,0,1);
    let loopback = Ip_Addr2::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number :Option<i32> = None;

}

fn route(ip_kind: IpAddrKind){}
