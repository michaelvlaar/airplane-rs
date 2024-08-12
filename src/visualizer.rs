use crate::weight_and_balance::{Airplane, Mass, Volume};
use core::ops::Range;
use plotters::{prelude::*, style::full_palette::{GREY, PURPLE}};

pub enum Visualization {
    Svg(String),
}

pub struct WeightBalanceChartVisualization {
    dimensions: (u32, u32),
    axis: (Range<f64>, Range<f64>),
}

impl WeightBalanceChartVisualization {
    pub fn new(
        dimensions: (u32, u32),
        axis: (Range<f64>, Range<f64>),
    ) -> WeightBalanceChartVisualization {
        WeightBalanceChartVisualization { dimensions, axis }
    }
}

pub struct WeightBalanceTableVisualization {
    dimensions: (u32, u32),
}

pub fn weight_and_balance_table_strings(plane: Airplane) -> Vec<Vec<String>> {
    let mut table = vec![vec![
        "Name".to_string(),
        "Lever Arm [m]".to_string(),
        "Mass [kg]".to_string(),
        "Mass Moment [kg m]".to_string(),
    ]];

    for m in plane.moments().iter() {
        table.push(vec![
            match m.mass() {
                Mass::Avgas(_) | Mass::Mogas(_) => format!("{} ({})", m.name(), m.mass().unit()).replace('.', ","),
                _ => m.name().clone(),
            },
            format!("{:.4}", m.lever_arm().meter()).replace('.', ","),
            match m.mass() {
                Mass::Avgas(v) | Mass::Mogas(v) => format!("({}) {:.2}", v.to_string(), m.mass().kilo()).replace('.', ","),
                _ => format!("{:.2}", m.mass().kilo()).replace('.', ","),
            },
            format!("{:.2}", m.total().kgm()).replace('.', ","),
        ])
    }

    table.push(vec![
        "Total".to_string(),
        format!(
            "{:.4}",
            plane.total_mass_moment().kgm() / plane.total_mass().kilo()
        ).replace('.', ","),
        format!("{:.2}", plane.total_mass().kilo()).replace('.', ","),
        format!("{:.2}", plane.total_mass_moment().kgm()).replace('.', ","),
    ]);

    table
}
impl WeightBalanceTableVisualization {
    pub fn new(dimensions: (u32, u32)) -> WeightBalanceTableVisualization {
        WeightBalanceTableVisualization { dimensions }
    }
}
pub fn weight_and_balance_table(
    plane: Airplane,
    visualization: WeightBalanceTableVisualization,
) -> Visualization {
    let mut rbuf = String::new();
    {
        let right = SVGBackend::with_string(
            &mut rbuf,
            (visualization.dimensions.0, visualization.dimensions.1),
        )
        .into_drawing_area();

        right
            .fill(&WHITE)
            .expect("cannot fill background with white.");

        let font = ("monospace", 20).into_font();
        let bold_font = ("monospace", 20).into_font().style(FontStyle::Bold);
        let text_style = TextStyle::from(font).color(&BLACK);
        let bold_text_style = TextStyle::from(bold_font).color(&BLACK);

        let cell_width = [110, 140, 180, 180];
        let cell_padding = [10, 70, 21, 114];
        let cell_height = 30;

        let start_x = 0;
        let start_y = 0;

        // Function to pad with non-breaking spaces
        fn pad_with_nbsp(text: &str, total_length: usize) -> String {
            let nb_spaces = total_length.saturating_sub(text.len());
            let mut padded = String::new();
            for _ in 0..nb_spaces {
                padded.push('\u{00A0}');
            }
            padded.push_str(text);
            padded
        }

        // Draw header row with grey background and bold text
        let total_width: i32 = cell_width.iter().sum();
        right
            .draw(&Rectangle::new(
                [
                    (start_x, start_y),
                    (start_x + total_width, start_y + cell_height),
                ],
                ShapeStyle {
                    color: GREY.mix(0.5).to_rgba(),
                    filled: true,
                    stroke_width: 0,
                },
            ))
            .expect("cannot draw header rectangle");

        right
            .draw_text("Name", &bold_text_style, (start_x + 10, start_y + 10))
            .expect("cannot draw text");

        let mut current_cell_width = start_x + cell_width[0];
        right
            .draw_text(
                "Lever Arm [m]",
                &bold_text_style,
                (current_cell_width + 10, start_y + 10),
            )
            .expect("cannot draw text");

        current_cell_width += cell_width[1];
        right
            .draw_text(
                "Mass [kg]",
                &bold_text_style,
                (current_cell_width + 80, start_y + 10),
            )
            .expect("cannot draw text");

        current_cell_width += cell_width[2];
        right
            .draw_text(
                "Mass Moment [kg m]",
                &bold_text_style,
                (current_cell_width + 10, start_y + 10),
            )
            .expect("cannot draw text");

        // Draw the rest of the table rows
        for (i, m) in plane.moments().iter().enumerate() {
            let y = start_y + (i as i32 + 1) * cell_height;

            right
                .draw_text(m.name(), &text_style, (start_x + cell_padding[0], y + 10))
                .expect("cannot draw text");

            let mut current_cell_width = start_x + cell_width[0];
            right
                .draw_text(
                    &pad_with_nbsp(&format!("{:.4}", m.lever_arm().meter()), 6),
                    &text_style,
                    (current_cell_width + cell_padding[1], y + 10),
                )
                .expect("cannot draw text");

            current_cell_width += cell_width[1];

            let mass_str = match m.mass() {
                Mass::Avgas(Volume::Liter(l)) => format!("({:.1}L) {:.2}", l, m.mass().kilo()),
                Mass::Avgas(Volume::Gallon(g)) => format!("({:.1}gal) {:.2}", g, m.mass().kilo()),
                Mass::Mogas(Volume::Liter(l)) => format!("({:.1}L) {:.2}", l, m.mass().kilo()),
                Mass::Mogas(Volume::Gallon(g)) => format!("({:.1}gal) {:.2}", g, m.mass().kilo()),
                Mass::Kilo(_) => format!("{:.2}", m.mass().kilo()),
            };

            right
                .draw_text(
                    &pad_with_nbsp(&mass_str, 16),
                    &text_style,
                    (current_cell_width + cell_padding[2], y + 10),
                )
                .expect("cannot draw text");

            current_cell_width += cell_width[2];
            right
                .draw_text(
                    &pad_with_nbsp(&format!("{:.2}", m.total().kgm()), 6),
                    &text_style,
                    (current_cell_width + cell_padding[3], y + 10),
                )
                .expect("cannot draw text");
        }

        let y = start_y + (plane.moments().len() + 1) as i32 * cell_height;

        // Draw footer row with grey background and bold text
        right
            .draw(&Rectangle::new(
                [(start_x, y), (start_x + total_width, y + cell_height)],
                ShapeStyle {
                    color: GREY.mix(0.5).to_rgba(),
                    filled: true,
                    stroke_width: 0,
                },
            ))
            .expect("cannot draw footer rectangle");

        right
            .draw_text(
                "Total",
                &bold_text_style,
                (start_x + cell_padding[0], y + 10),
            )
            .expect("cannot draw text");

        let mut current_cell_width = start_x + cell_width[0];
        right
            .draw_text(
                &pad_with_nbsp(
                    &format!(
                        "{:.4}",
                        plane.total_mass_moment().kgm() / plane.total_mass().kilo()
                    ),
                    6,
                ),
                &bold_text_style,
                (current_cell_width + cell_padding[1], y + 10),
            )
            .expect("cannot draw text");

        current_cell_width += cell_width[1];
        right
            .draw_text(
                &pad_with_nbsp(&format!("{:.2}", plane.total_mass().kilo()), 16),
                &bold_text_style,
                (current_cell_width + cell_padding[2], y + 10),
            )
            .expect("cannot draw text");

        current_cell_width += cell_width[2];
        right
            .draw_text(
                &pad_with_nbsp(&format!("{:.2}", plane.total_mass_moment().kgm()), 6),
                &bold_text_style,
                (current_cell_width + cell_padding[3], y + 10),
            )
            .expect("cannot draw text");

        // Draw horizontal lines for the table
        for i in 0..=(plane.moments().len() + 2) {
            let y = start_y + i as i32 * cell_height;
            right
                .draw(&PathElement::new(
                    vec![(start_x, y), (start_x + total_width, y)],
                    BLACK,
                ))
                .expect("cannot draw lines");
        }

        // Draw vertical lines for the table
        let mut x = start_x;
        right
            .draw(&PathElement::new(
                vec![
                    (x, start_y),
                    (
                        x,
                        start_y + cell_height * (plane.moments().len() + 2) as i32,
                    ),
                ],
                BLACK,
            ))
            .expect("cannot draw lines");

        for j in cell_width.iter() {
            x += j;
            right
                .draw(&PathElement::new(
                    vec![
                        (x, start_y),
                        (
                            x,
                            start_y + cell_height * (plane.moments().len() + 2) as i32,
                        ),
                    ],
                    BLACK,
                ))
                .expect("cannot draw lines");
        }

        right.present().expect("cannot write to buffer.");
    }

    Visualization::Svg(rbuf)
}

pub fn weight_and_balance_chart(
    plane: Airplane,
    visualization: WeightBalanceChartVisualization,
) -> Visualization {
    let mut lbuf = String::new();

    {
        let left = SVGBackend::with_string(
            &mut lbuf,
            (visualization.dimensions.0, visualization.dimensions.1),
        )
        .into_drawing_area();

        left.fill(&WHITE)
            .expect("cannot fill background with white.");

        let mut chart = ChartBuilder::on(&left)
            .caption(plane.callsign(), ("sans-serif", 50).into_font())
            .margin(5)
            .margin_right(20)
            .x_label_area_size(50)
            .y_label_area_size(80)
            .build_cartesian_2d(visualization.axis.0.clone(), visualization.axis.1.clone())
            .expect("cannot build chart.");

        chart
            .configure_mesh()
            .x_desc("Mass Moment [kg m]")
            .x_label_style(("sans-serif", 20).into_font())
            .y_desc("Mass [kg]")
            .y_label_style(("sans-serif", 20).into_font())
            .x_label_formatter(&|x| format!("{}", x.round()))
            .y_label_formatter(&|y| format!("{}", y.round()))
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

        // Draw the square (CG envelope)
        chart
            .draw_series(std::iter::once(Polygon::new(square_points, RED.mix(0.2))))
            .expect("cannot draw polygon.")
            .label("CG Envelope")
            .legend(|(x, y)| Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], RED.mix(0.2).filled()));

        // Draw the total mass and moment point
        chart
            .draw_series(PointSeries::of_element(
                vec![(plane.total_mass_moment().kgm(), plane.total_mass().kilo())],
                5,
                if plane.within_limits() { GREEN } else { RED },
                &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
            ))
            .expect("cannot draw point.")
            .label("Take-off Point")
            .legend(|(x, y)| Circle::new((x, y), 5, GREEN.filled()));

        // Draw the landing mass and moment point
        chart
            .draw_series(PointSeries::of_element(
                vec![(plane.total_mass_moment_landing().kgm(), plane.total_mass_landing().kilo())],
                5,
                PURPLE,
                &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
            ))
            .expect("cannot draw point.")
            .label("Landing Point")
            .legend(|(x, y)| Circle::new((x, y), 5, PURPLE.filled()));

        // Configure and draw the legend
        chart
            .configure_series_labels()
            .border_style(BLACK)
            .margin(20)
            .background_style(WHITE.mix(0.8))
            .draw()
            .expect("cannot draw legend");

        left.present().expect("cannot write to buffer.");
    }

    Visualization::Svg(lbuf)
}

//pub fn weight_and_balance_chart(
//    plane: Airplane,
//    visualization: WeightBalanceChartVisualization,
//) -> Visualization {
//    let mut lbuf = String::new();
//
//    {
//        let left = SVGBackend::with_string(
//            &mut lbuf,
//            (visualization.dimensions.0, visualization.dimensions.1),
//        )
//        .into_drawing_area();
//
//        left.fill(&WHITE)
//            .expect("cannot fill background with white.");
//
//        let mut chart = ChartBuilder::on(&left)
//            .caption(plane.callsign(), ("sans-serif", 50).into_font())
//            .margin(5)
//            .margin_right(20)
//            .x_label_area_size(50)
//            .y_label_area_size(80)
//            .build_cartesian_2d(visualization.axis.0.clone(), visualization.axis.1.clone())
//            .expect("cannot build chart.");
//
//        chart
//            .configure_mesh()
//            .x_desc("Mass Moment [kg m]")
//            .x_label_style(("sans-serif", 20).into_font())
//            .y_desc("Mass [kg]")
//            .y_label_style(("sans-serif", 20).into_font())
//            .x_label_formatter(&|x| format!("{}", x.round()))
//            .y_label_formatter(&|y| format!("{}", y.round()))
//            .draw()
//            .expect("cannot configure mesh.");
//
//        let kg_mtow = plane.limits().mtow().kilo();
//        let m_forward_cg_moment = plane.limits().forward_cg_limit().meter();
//        let m_rearward_cg_moment = plane.limits().rearward_cg_limit().meter();
//        let kg_minimum_weight = plane.limits().minimum_weight().kilo();
//        let square_points = vec![
//            (m_forward_cg_moment * kg_minimum_weight, kg_minimum_weight),
//            (m_rearward_cg_moment * kg_minimum_weight, kg_minimum_weight),
//            (m_rearward_cg_moment * kg_mtow, kg_mtow),
//            (m_forward_cg_moment * kg_mtow, kg_mtow),
//        ];
//
//        chart
//            .draw_series(std::iter::once(Polygon::new(square_points, RED.mix(0.2))))
//            .expect("cannot draw polygon.");
//
//        chart
//            .draw_series(PointSeries::of_element(
//                vec![(plane.total_mass_moment().kgm(), plane.total_mass().kilo())],
//                5,
//                if plane.within_limits() { GREEN } else { RED },
//                &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
//            ))
//            .expect("cannot draw point.");
//
//        chart
//            .draw_series(PointSeries::of_element(
//                vec![(plane.total_mass_moment_landing().kgm(), plane.total_mass_landing().kilo())],
//                5,
//                PURPLE,
//                &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
//            ))
//            .expect("cannot draw point.");
//        left.present().expect("cannot write to buffer.");
//    }
//
//    Visualization::Svg(lbuf)
//}
