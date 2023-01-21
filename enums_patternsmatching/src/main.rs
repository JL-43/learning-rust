fn route(ip_kind:IpAddrKind){
    
}

fn main() {

    //enums keep values between certain options

    // in our example, we will create an enum for IP addressed (only two possible kinds: IPv4 and IPv6)

    enum IpAddrKind{
        V4,
        V6
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

}
