use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    println!("enter your weight: ");
    io::stdin().read_line(&mut input)?;
    let weight: f32 = input.trim().parse().unwrap();
    println!("{} kg on earth is {} kg on mars!", weight, calculate_weight_on_mars(weight));
    Ok(())
}

const G_EARTH: f32 = 9.81;
const G_MARS: f32 = 3.711;

fn calculate_weight_on_mars(weight_on_earth: f32) -> f32 {
    weight_on_earth / G_EARTH * G_MARS
}
