#[derive(Debug)]
enum StatusMessage{
    Ok,
}

#[derive(Debug)]
struct CubeSat{
    id: u64,
}

fn chech_status(sat_id: CubeSat) -> StatusMessage{
    StatusMessage::Ok
}

fn main() {
    let sat_a = CubeSat{ id: 0};
    let sat_b = CubeSat{ id: 1};
    let sat_c = CubeSat{ id: 2};
    
    let a_status = chech_status(sat_a);
    let b_status = chech_status(sat_b);
    let c_status = chech_status(sat_c);
    println!("a: {:?}, b:{:?}, c:{:?}.", a_status, b_status,c_status);
    
    //waiting...
    let a_status = chech_status(sat_a);
    let b_status = chech_status(sat_b);
    let c_status = chech_status(sat_c);
    println!("a: {:?}, b:{:?}, c:{:?}.", a_status, b_status,c_status);
}
