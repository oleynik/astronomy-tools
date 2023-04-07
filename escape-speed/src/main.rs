use escape_speed::*;
use std::env;

fn supported_velocities() -> Vec<&'static str> {
    vec!["v1"]
}

fn validation(args: &Vec<String>) -> Result<(), String> {
    if args.len() != 4 {
        return Err(String::from("Unexpected number of arguments."));
    }
    if !solar_system()
        .iter()
        .map(|e| e.name())
        .any(|n| n == args[1])
    {
        return Err(String::from("Unknown name of Heavenly Object."));
    }
    let velocities = supported_velocities();
    if !velocities.contains(&args[2].as_str()) {
        return Err(format!(
            "Unsupported velocity. Supported velocities are: [{}]",
            velocities.join(" ")
        ));
    }
    match args[3].parse::<f64>() {
        Ok(v) if v >= 0_f64 => Ok(()),
        Ok(v) => Err(format!("Distance cannot be negative: {}", v)),
        Err(e) => Err(e.to_string()),
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    validation(&args)?;

    let name = args[1].clone();
    let velocity = args[2].clone();
    let distance = args[3].parse::<f64>().unwrap();
    let vec = solar_system();
    let heavenly_object = vec.iter().find(|&o| o.name() == name).unwrap();
    match velocity.as_str() {
        "v1" => println!(
            "Escape Velocity is {:.2e} m/s.",
            heavenly_object.escape_speed(distance)
        ),
        v => panic!("Unknown velocity {}", v),
    }
    Ok(())
}
