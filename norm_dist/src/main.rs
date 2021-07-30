use rand::{
    thread_rng,
    prelude::SliceRandom
};

use plotters::{
    style::IntoFont,
    drawing::IntoDrawingArea,
    prelude::{
        SVGBackend, ChartBuilder, LineSeries, BLUE, PointSeries, RED, EmptyElement, Circle}
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let coords = vector_make();
    let root = SVGBackend::new("7.svg", (1920, 1080)).into_drawing_area();
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("Probability of Coin Flip over Time", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f32..3000f32, -1f32..1f32)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(25)
        .y_labels(50)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;
    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(
        vec![(0.0, 0.0), (3000., 0.0)],
        &BLUE,
    ))?;    

    // Similarly, we can draw point series
    chart.draw_series(PointSeries::of_element(
        coords,
        1,
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
            // + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
        },
    ))?;
    Ok(())
}

fn vector_make() -> Vec<(f32, f32)> {
    let mut point_series: Vec<(f32, f32)> = Vec::new();
    let mut ylist: Vec<f32> = Vec::new();
    let types = [-1.000000000000, 1.000000000000];
    for hxvalue in 1..3001 {
        let y: f32 = *types.choose(&mut thread_rng()).unwrap() as f32;
        ylist.push(y);
        let mut hyvalue: f32 = 0.000000000000;
        for i in &ylist {
            hyvalue += i
        };
        let xvalue: f32 = hxvalue as f32;
        let yvalue: f32 = hyvalue/xvalue;
        point_series.push((xvalue, yvalue));
    };
    point_series
}