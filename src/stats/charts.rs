use charming::{
    Chart, ImageFormat, ImageRenderer,
    component::{Axis, Grid, Legend, Title},
    element::{AxisPointer, AxisPointerType, AxisType, Tooltip, Trigger},
    series::Bar,
    theme::Theme,
};
use std::time;

pub fn chart(
    filename: &str,
    sample_labels: &Vec<&str>,
    total_ts: &Vec<time::Duration>,
    is_concurrent: bool,
) -> std::io::Result<()> {
    let c = Chart::new()
        .title(Title::new().text("Render Profile"))
        .tooltip(
            Tooltip::new()
                .trigger(Trigger::Axis)
                .axis_pointer(AxisPointer::new().type_(AxisPointerType::Shadow)),
        )
        .legend(Legend::new())
        .grid(
            Grid::new()
                .left("3%")
                .right("4%")
                .bottom("3%")
                .contain_label(true),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Value)
                .boundary_gap(("0", "0.01")),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(sample_labels.clone()),
        )
        .series(
            Bar::new()
                .name("Total Time (s)")
                .data(total_ts.iter().map(|t| t.as_secs() as i32).collect()),
        );

    let mut renderer = ImageRenderer::new(1000, 800).theme(Theme::Vintage);
    let chart_filename = if is_concurrent {
        format!("profile/profile_{}_concurrent.png", filename)
    } else {
        format!("profile/profile_{}.png", filename)
    };
    match renderer.save_format(ImageFormat::Png, &c, &chart_filename) {
        Ok(_) => Ok(()),
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
    }
}
