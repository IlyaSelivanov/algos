fn main() {
    for i in 0..100 {
        let x = 8f64 * i as f64 * i as f64;
        let y = 64f64 * i as f64 * (i as f64).log2();

        println!("i = {}; x = {:.3}; y = {:.3}", i, x, y);
    }

    println!("{}", (60f64 * 1_000_000f64).log2());
    println!("{}", 2f64.powf(25.84f64));
}
