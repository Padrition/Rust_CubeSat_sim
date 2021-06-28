#[derive(Debug)]
enum StatusMessage{
    Ok,
}

fn chech_status(sat_id: u64) -> StatusMessage{
    StatusMessage::Ok
}

fn main() {
    let sat_a = 0;
    let sat_b = 1;
    let sat_c = 2;
    
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
