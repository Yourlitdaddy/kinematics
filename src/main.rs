mod kinematics;
use crate::kinematics::kinematics;

use std::{io, str::FromStr};

use dialoguer::Input;
use terminal_size::{terminal_size, Height, Width};

fn get_terminal_size() -> Option<(u16, u16)> {
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        return Some((w, h));
    } else {
        return None;
    }
}

fn str_to_float(str: String) -> Option<f64> {
    if str == "none" {
        return None;
    } else {
        return Some(str.parse::<f64>().unwrap());
    }
}

fn main() -> Result<(), io::Error> {
    let mut w: usize = 0;
    match get_terminal_size() {
        Some((width, _height)) => {
            w = width.into();
        }
        None => println!("Unable to get terminal size"),
    }

    println!("{:^w$}", "\n\n\nKinematics Solver\n\n\n");

    let a_input: String = Input::<String>::new()
        .with_prompt("Acceleration")
        .default("none".into())
        .interact_text()?;

    let d_input: String = Input::<String>::new()
        .with_prompt("Displacement")
        .default("none".into())
        .interact_text()?;

    let vi_input: String = Input::<String>::new()
        .with_prompt("Initial Velocity")
        .default("none".into())
        .interact_text()?;

    let vf_input: String = Input::<String>::new()
        .with_prompt("Final Velocity")
        .default("none".into())
        .interact_text()?;

    let t_input: String = Input::<String>::new()
        .with_prompt("Time")
        .default("none".into())
        .interact_text()?;

    kinematics(
        str_to_float(a_input),
        str_to_float(d_input),
        str_to_float(vi_input),
        str_to_float(vf_input),
        str_to_float(t_input),
    );

    Ok(())
}
