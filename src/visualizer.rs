use crate::weight_and_balance::Airplane;
use core::ops::Range;
use plotters::prelude::*;

pub enum Visualization {
    Svg(String),
}

pub struct WeightBalanceVisualization {
    dimensions: (u32, u32),
    axis: (Range<f64>, Range<f64>),
}

impl WeightBalanceVisualization {
    pub fn new(
        dimensions: (u32, u32),
        axis: (Range<f64>, Range<f64>),
    ) -> WeightBalanceVisualization {
        WeightBalanceVisualization { dimensions, axis }
    }
}

pub fn weight_and_balance(
    plane: Airplane,
    visualization: WeightBalanceVisualization,
) -> Visualization {
    let mut buf = String::new();

    {
        let root = SVGBackend::with_string(
            &mut buf,
            (visualization.dimensions.0, visualization.dimensions.1),
        )
        .into_drawing_area();
        root.fill(&WHITE)
            .expect("cannot fill background with white.");

        let mut chart = ChartBuilder::on(&root)
            .caption(plane.callsign(), ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(50)
            .y_label_area_size(80)
            .build_cartesian_2d(visualization.axis.0, visualization.axis.1)
            .expect("cannot build chart.");

        chart
            .configure_mesh()
            .x_desc("Mass Moment [kg m]")
            .x_label_style(("sans-serif", 20).into_font())
            .y_desc("Mass [kg]")
            .y_label_style(("sans-serif", 20).into_font())
            .draw()
            .expect("cannot configure mesh.");

        let kg_mtow = plane.limits().mtow().kilo();
        let m_forward_cg_moment = plane.limits().forward_cg_limit().meter();
        let m_rearward_cg_moment = plane.limits().rearward_cg_limit().meter();
        let kg_minimum_weight = plane.limits().minimum_weight().kilo();
        let square_points = vec![
            (m_forward_cg_moment * kg_minimum_weight, kg_minimum_weight),
            (m_rearward_cg_moment * kg_minimum_weight, kg_minimum_weight),
            (m_rearward_cg_moment * kg_mtow, kg_mtow),
            (m_forward_cg_moment * kg_mtow, kg_mtow),
        ];

        chart
            .draw_series(std::iter::once(Polygon::new(square_points, RED.mix(0.2))))
            .expect("cannot draw polygon.");

        chart
            .draw_series(PointSeries::of_element(
                vec![(plane.total_mass_moment().kgm(), plane.total_mass().kilo())],
                5,
                if plane.within_limits() { GREEN } else { RED },
                &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
            ))
            .expect("cannot draw point.");

        root.present().expect("cannot write to buffer.");
    }

    Visualization::Svg(buf)
}
