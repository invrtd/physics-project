use std::f32::consts::PI;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::LineStyle;

const GRAVITY:f32 = -9.805;
const MASS:f32 = 6.0;
const RADIUS:f32 = 0.011;
const DT:f32 = 0.001;

struct graph_properties {

    plot_points:Vec<(f64, f64)>,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    x_axis_name: String,
    y_axis_name: String


}


fn main() {
    let output = compute_data(); //[t, h, v, a, drag]

    let time = get_colunm(0,&output);
    let height = get_colunm(1,&output);
    let velocity = get_colunm(2,&output);
    let acceleration = get_colunm(3,&output);
    let drag = get_colunm(4,&output);
   /* let graph_height_time = graph_properties{
        plot_points:,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        x_axis_name: String,
        y_axis_name: String


    }
    */





}

impl graph_properties {
    fn plot (data: Vec<(f64, f64)>,x_axis_name: String, y_axis_name: String,x_bounds:(f64,f64),y_bounds:(f64,f64)) {
        // We create our scatter plot from the data
        let s1: Plot = Plot::new(data).line_style(
            LineStyle::new()
                .colour("#DD3355"),
        ); // and a custom colour

        // The 'view' describes what set of data is drawn
        let v = ContinuousView::new()
            .add(s1)

            .x_range(0., 50.)
            .y_range(0., 25.)
            .x_label(x_axis_name)
            .y_label(y_axis_name);

        // A page with a single view is then saved to an SVG file
        Page::single(&v).save("scatter.svg").unwrap();
    }
}

fn get_bounds (input: Vec<(f64)>)-> (f64,f64) {

    let min_value = *input.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_value = *input.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();


    return (min_value,max_value)
}


fn parse_for_graph (i: usize, input: &Vec<f32>) -> Vec<(f64,f64)> {

    let mut return_vector = vec![];
    for row in input{
        let temp = (row[0].into(),row[i].into());
        return_vector.push(temp);
    }

    return return_vector ;
}


fn get_colunm (i: usize, input: &Vec<f32>) -> Vec<(f64)> {

    let mut return_vector = vec![];
    for row in input{
        let temp = (row[i].into());
        return_vector.push(temp);
    }

return return_vector ;
}

fn compute_data () -> Vec<[f32;5]> {


    let mut h :f32 = 440.0;
    let viscosity :f32 = 14.8;
    let mut v :f32 = 0.0;
    let mut t :f32 = 0.0;
    let mut a :f32 = 0.0;
    let mut drag :f32 = 0.0;
    // let density :f32 = 1.225;
    let mut output =vec![[t, h, v, a, drag]];


    while h >= 0.0 {

        drag = get_drag(v,  viscosity);
        a = update_a(drag);
        v = update_v(v, a);
        h = update_h(h, v);
        t = step_time(t);
        output.extend([[t, h, v.abs(), a, drag]]);
        //println!("[{},{},{},{},{}]", t, h, v, a, drag);
    }

return output

}

fn step_time (t :f32) -> f32{

    t + DT

}
fn update_h (h :f32, v :f32) -> f32{

    h + v * DT

}
fn update_v (v :f32, a :f32,) -> f32{

    v + a * DT

}
fn update_a (drag :f32) -> f32{

    net_force(force(GRAVITY), drag) / MASS

}

fn force(acceleration :f32) -> f32 {
    MASS * acceleration
}

fn get_drag(v :f32, viscosity :f32) -> f32 {
    6.0 * PI * RADIUS * viscosity * v.abs()
}

fn net_force(force_one :f32, force_two :f32) -> f32 {
    force_one + force_two
}
