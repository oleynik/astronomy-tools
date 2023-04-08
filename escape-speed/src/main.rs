use escape_speed::{solar_system, HeavenlyBody};
use std::env;

fn supported_velocities() -> Vec<&'static str> {
    vec!["v1"]
}

fn v_number_of_arguments(n: usize) -> Result<(), String> {
    if n != 4 {
        Err(String::from("Unexpected number of arguments."))
    } else {
        Ok(())
    }
}
fn v_objects_name(name: &String) -> Result<(), String> {
    if !solar_system().iter().map(|e| e.name()).any(|n| n == *name) {
        return Err(String::from("Unknown name of Heavenly Object."));
    } else {
        Ok(())
    }
}
fn v_velocity(velocity: &String) -> Result<(), String> {
    let velocities = supported_velocities();
    if !velocities.contains(&velocity.as_str()) {
        return Err(format!(
            "Unsupported velocity. Supported velocities are: [{}]",
            velocities.join(" ")
        ));
    } else {
        Ok(())
    }
}
fn v_distance(distance: &String) -> Result<(), String> {
    match distance.parse::<f64>() {
        Ok(v) if v >= 0_f64 => Ok(()),
        Ok(v) => Err(format!("Distance cannot be negative: {}", v)),
        Err(e) => Err(e.to_string()),
    }
}

fn validation(args: &Vec<String>) -> Result<(), String> {
    v_number_of_arguments(args.len())?;
    v_objects_name(&args[1])?;
    v_velocity(&args[2])?;
    v_distance(&args[3])?;
    Ok(())
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
