use escape_speed::*;

fn main() {
    solar_system().iter().for_each(|ho| {
        println!("{:?}", ho);
        println!("Escape Speed: {}", ho.escape_speed(0_f64));
        println!();
    })
}
