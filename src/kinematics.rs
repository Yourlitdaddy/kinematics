use libm::{pow, sqrt};

pub fn kinematics(
    mut a: Option<f64>,
    mut d: Option<f64>,
    mut vi: Option<f64>,
    mut vf: Option<f64>,
    mut t: Option<f64>,
) {
    if d.is_none() {
        if vi.is_some() && vf.is_some() && t.is_some() {
            let sum = ((vi.unwrap() + vf.unwrap()) / 2.0) * t.unwrap();
            d = Some(sum);
            println!("d = {:?}", d.unwrap());
        } else if vi.is_some() && t.is_some() && a.is_some() {
            let ans = (vi.unwrap() * t.unwrap()) + 0.5 * (a.unwrap() * (pow(t.unwrap(), 2.0)));
            d = Some(ans);
            println!("d = {:?}", d.unwrap());
        }
    }
    if a.is_none() {
        if vf.is_some() && t.is_some() && vi.is_some() {
            let ans = (vf.unwrap() - vi.unwrap()) / t.unwrap();
            a = Some(ans);
            println!("a = {:?}", a.unwrap());
        } else if vf.is_some() && vi.is_some() && d.is_some() {
            let ans = pow(vf.unwrap(), 2.0) - pow(vi.unwrap(), 2.0) / (d.unwrap() * 2.0);
            a = Some(ans);
            println!("a = {:?}", a.unwrap());
        }
    }
    if vf.is_none() {
        if a.is_some() && vi.is_some() && t.is_some() {
            let ans = sqrt(pow(vi.unwrap(), 2.0) + 2.0 * a.unwrap() * d.unwrap());
            vf = Some(ans);
            println!("vf = ±{:?}", vf.unwrap());
        }
    }
    if vi.is_none() {
        if a.is_some() && t.is_some() && vf.is_some() {
            let ans = vf.unwrap() - (a.unwrap() / t.unwrap());
            vi = Some(ans);
            println!("vi = {:?}", vi.unwrap());
        }
    }
    if t.is_none() {
        if a.is_some() && d.is_some() && vi.is_some() {
            let ans = sqrt(d.unwrap() / (vi.unwrap() + (0.5 * a.unwrap())));
            t = Some(ans);
            println!("t = {:?}", t.unwrap());
        }
    }

    if a.is_none() || d.is_none() || vf.is_none() || vi.is_none() || t.is_none() {
        kinematics(a, d, vi, vf, t);
    }
}
