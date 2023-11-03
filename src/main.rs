use std::f64::consts::PI;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::LineStyle;
const GRAVITY: f64 = -9.805;
const MASS: f64 = 6.0;
const RADIUS: f64 = 0.011;
const DT: f64 = 0.001;

struct GraphProperties {
    plot_points: Vec<(f64,f64)>,
    x_axis_name: String,
    y_axis_name: String,
}
fn main() {
    let(time_height, height_data,velocity_data,acceleration_data,drag_data) = compute_data();
    // time height is only fuctional one currently


    let test = GraphProperties {
        plot_points: (time_height),
        x_axis_name: "it works?".parse().unwrap(),
        y_axis_name: "it works?".parse().unwrap()
    };
    test.plot()


}








impl GraphProperties {
    fn plot (self) {

        // We create our scatter plot from the data
        let s1: Plot = Plot::new(self.plot_points).line_style(
            LineStyle::new()
                .colour("#DD3355"),
        ); // and a custom colour

        // The 'view' describes what set of data is drawn
        let v = ContinuousView::new()
            .add(s1)

            .x_range(0., 25.)
            .y_range(0., 60.0)
            .x_label(&self.x_axis_name)
            .y_label(&self.y_axis_name);

        // A page with a single view is then saved to an SVG file
        Page::single(&v).save("scatter.svg").unwrap();
    }
}






fn graph_range (input: Vec<f64>) -> (f64,f64){
    let mut min = 0.0 ;
    let mut max = 0.0 ;
    if input[0]>input[input.len()-1] {
        max = input[0];
        min = input[input.len()-1];
    } else {
        min = input[0];
        max = input[input.len()-1];
        return     (min,max);
    }
    return     (min,max);
}
fn compute_data() -> (Vec<(f64, f64)>,Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>) {
    let mut h: f64 = 440.0;
    let viscosity: f64 = 14.8;
    let mut v: f64 = 0.0;
    let mut t: f64 = 0.0;
    let mut a: f64 = 0.0;
    let mut drag: f64 = 0.0;
    let mut time_height = vec![(t,v)];
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
        time_height.push((t,drag.abs()));
        /*height_data.push([h]);
        velocity_data.push([v]);
        acceleration_data.push([a]);
        drag_data.push([drag]);
        */
    }

    return (time_height,height_data,velocity_data,acceleration_data,drag_data);
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
