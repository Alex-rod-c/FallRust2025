const freeze_F: f64 = 32.0;

fn far_2_cel(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn cel_2_far(c: f64) -> f64{
    (c * 9.0 / 5.0) + 32.0
}

fn main(){
    let mut temp_f: f64 = freeze_F;
    pintln!("{:.1}°F = {:.1}°C", temp_f, far_2_cel(temp_f));

    for i in 1..=5 {
        temp_f += 1.0;
        println!("{:.1}°F = {:.1}°", temp_f, far_2_cel(temp_f));
    }
}
