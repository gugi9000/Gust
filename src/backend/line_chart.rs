
use backend::elements::line_chart::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use backend::traits::Graphable;

pub struct LineChart {
    identifier: String,
    description: String,
    width: u32,
    height: u32,
    padding: u32,
    signals: Vec<LineChartSignal>,
    data: Vec<LineChartData>,
    scales: Vec<LineChartScale>,
    axes: Vec<LineChartAxis>,
    marks: Vec<LineChartMark>,
}

impl LineChart {
    pub fn new() -> LineChart {
        LineChart {
            identifier: String::from("line_chart"),
            description: String::from("Line Chart"),
            width: 500,
            height: 300,
            padding: 5,
            signals: vec![LineChartSignal::new()],
            data: vec![LineChartData::new()],
            scales: vec![
                LineChartScale::new_xscale(),
                LineChartScale::new_yscale(),
                LineChartScale::new_ordinal_scale(),
            ],
            axes: vec![LineChartAxis::new_xaxis(), LineChartAxis::new_yaxis()],
            marks: vec![LineChartMark::new()],
        }
    }
    /// To add data to a line chart, the data must be formatted in the following fashion:
    /// {Integer, Integer, Integer }.
    ///
    /// The first two entries represent the x and y coordinates of the point
    /// which you're adding to the graph, and the third coordinate is the series identifier.
    /// For example, if you want to add 2 different lines on a single set of axes, then you can
    /// set the z of the first series to 0, and set the z of the second series to 1.
    pub fn add_data(&mut self, x: i64, y: i64, z: i64) {
        self.data[0].add_data(x, y, z);
    }
}
impl Serialize for LineChart {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("line_chart", 10)?;
        s.serialize_field("$schema", "https://vega.github.io/schema/vega/v3.0.json")?;
        s.serialize_field("width", &self.width)?;
        s.serialize_field("height", &self.height)?;
        s.serialize_field("padding", &self.padding)?;
        s.serialize_field("signals", &self.signals)?;
        s.serialize_field("data", &self.data)?;
        s.serialize_field("scales", &self.scales)?;
        s.serialize_field("axes", &self.axes)?;
        s.serialize_field("marks", &self.marks)?;

        s.end()
    }
}
impl Graphable for LineChart {
    fn get_description(&self) -> &str {
        &self.description
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
}