// fn main() {
//     #[derive(Debug)] 
//     struct Ipv4Addr {
//         // ....
//     }

//     #[derive(Debug)]
//     struct Ipv6Addr {
//         // ....
//     }

//     #[derive(Debug)]
//     enum IpAddr {
//         V4(Ipv4Addr),
//         V6(Ipv6Addr),
//     }

//     let v4 = IpAddr::V4(Ipv4Addr{});
//     let v6 = IpAddr::V6(Ipv6Addr{});

//     println!("{:?}", v4);
//     println!("{:?}", v6);
// }

fn main() {
    let num1: i8 = 5;
    let num2: Option<i8> = Some(3);
    let num3: Option<i8> = None;

    handle_options(num1, num2);
    handle_options(num1, num3);
}

fn handle_options(value: i8, optional_value: Option<i8>) {
    match optional_value {
        Option::Some(optional_value) => {
            println!("Some optional_value: {}", optional_value);
            let res = value + optional_value;

            println!("Result: {}", res);
        },
        Option::None => {
            println!("None");
        }
    }
}
