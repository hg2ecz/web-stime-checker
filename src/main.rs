mod meas; // mérjünk
mod plot; // ábrázoljunk
mod stat; // statisztikázzunk

use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Paraméterek: grafikonnév.png https://www.akarmi.hu THREADSZÁM MÁSODPERC");
    println!("    opcionálisan a végére egy 'rand' paraméter, amely ?tbench=véletlenszám tesztet csinál.");
    let args: Vec<_> = std::env::args().collect();
    let fname = &args[1];
    let url = args[2].to_string();
    let threads = args[3].parse().unwrap();
    let second = args[4].parse().unwrap();
    let mut rand = false;
    let mut rand_text = "".to_string();
    if args.get(5).is_some() && args[5] == "rand" {
        println!("--> randomteszt !!!");
        rand = true;
        rand_text = "(random)".to_string();
    }
    let res = meas::meas(&url, threads, second, rand);
    //println!("Adott thread lekérése: {:?}", res.0);
    //println!("Részletek:\n{:?}", res.1);
    let mut file = File::create("wtest-debug.log").unwrap();
    for r in &res.1 {
        file.write_all(format!("{:?}\n", r).as_bytes()).unwrap();
    }

    let stat1 = stat::min_max_time(&res.1, second as usize);
    println!("{:?}", stat1.0);
    println!("Err: {}", stat1.1);
    println!("Ok: {}", stat1.2);

    let _ = plot::plot_min_max_time(&fname, stat1.0, &url, threads, &rand_text);
}
