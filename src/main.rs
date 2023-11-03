use std::f64::consts::PI;

const GRAVITY: f64 = -9.805;
const MASS: f64 = 6.0;
const RADIUS: f64 = 0.011;
const DT: f64 = 0.001;


fn main() {
   let( time_data,height_data,velocity_data,acceleration_data,drag_data) = compute_data();
    println!("{:?}",height_data)


}
struct GraphProperties {
    plot_points: Vec<(f64, f64)>,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    x_axis_name: String,
    y_axis_name: String,
}
fn compute_data() -> (Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>) {
    let mut h: f64 = 440.0;
    let viscosity: f64 = 14.8;
    let mut v: f64 = 0.0;
    let mut t: f64 = 0.0;
    let mut a: f64 = 0.0;
    let mut drag: f64 = 0.0;
    let mut time_data = vec![t];
    let mut height_data = vec![h];
    let mut velocity_data = vec![v];
    let mut acceleration_data = vec![a];
    let mut drag_data = vec![drag];



    while h >= 0.0 {
        drag = get_drag(v, viscosity);
        a = update_a(drag);
        v = update_v(v, a);
        h = update_h(h, v);
        t = step_time(t);
        time_data.extend([t]);
        height_data.extend([h]);
        velocity_data.extend([v]);
        acceleration_data.extend([a]);
        drag_data.extend([drag]);
        
    }

    return (time_data,height_data,velocity_data,acceleration_data,drag_data);
}
fn step_time (t :f64) -> f64{

    t + DT

}
fn update_h (h :f64, v :f64) -> f64{

    h + v * DT

}
fn update_v (v :f64, a :f64,) -> f64{

    v + a * DT

}
fn update_a (drag :f64) -> f64{

    net_force(force(GRAVITY), drag) / MASS

}

fn force(acceleration :f64) -> f64 {
    MASS * acceleration
}

fn get_drag(v :f64, viscosity :f64) -> f64 {
    6.0 * PI * RADIUS * viscosity * v.abs()
}

fn net_force(force_one :f64, force_two :f64) -> f64 {
    force_one + force_two
}
