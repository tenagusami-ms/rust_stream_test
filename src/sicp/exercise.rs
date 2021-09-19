use super::iota::Iota;

pub fn stream_exercise(){
    let iota = Iota(0);
    let s = iota.take_while(|&i| i < 10 );
    for ss in s {
        println!("{}", ss);
    }

    let _a = 0;
}