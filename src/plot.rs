#![allow(unused)]

use std::{f64::{consts::{TAU, PI}, EPSILON}, ops::{Range, AddAssign, SubAssign}, fmt::Debug};

use linspace::LinspaceArray;
use num::{Complex, Float, traits::AsPrimitive, NumCast};
use plotters::{prelude::*, element::PointCollection, coord::{ranged3d::{ProjectionMatrixBuilder, ProjectionMatrix, Cartesian3d}, ranged1d::{ValueFormatter, DefaultFormatting, AsRangedCoord, NoDefaultFormatting}}, style::full_palette::PURPLE, chart::MeshStyle};

const PLOT_RES: (u32, u32) = (1024, 760);
const PLOT_CAPTION_FONT: (&str, u32) = ("sans", 20);
const PLOT_MARGIN: u32 = 5;
const PLOT_LABEL_AREA_SIZE: u32 = 30;

fn isometric(mut pb: ProjectionMatrixBuilder) -> ProjectionMatrix
{
    pb.yaw = core::f64::consts::FRAC_PI_4;
    pb.pitch = core::f64::consts::FRAC_PI_4;
    pb.scale = 0.7;
    pb.into_matrix()
}

pub fn plot_pz(
    plot_title: &str, plot_path: &str,
    poles: &[Complex<f64>],
    zeros: &[Complex<f64>]
) -> Result<(), Box<dyn std::error::Error>>
{
    const SYM_SIZE: f64 = 0.02;
    const PADDING: f64 = 3.0;
    const CIRCLE_RES: usize = 64;
    const POLE_SYM: [[Complex<f64>; 2]; 2] = [
        [Complex::new(-1.0, -1.0), Complex::new(1.0, 1.0)],
        [Complex::new(-1.0, 1.0), Complex::new(1.0, -1.0)]
    ];
    let zero_sym: [Complex<f64>; CIRCLE_RES] = (0.0..=TAU).linspace_array()
        .map2(/*const*/ |theta| Complex::new(theta.approx_cos(), theta.approx_sin()));

    let (sigma_min, sigma_max, omega_min, omega_max) = poles.into_iter()
        .chain(zeros.into_iter())
        .map(|c| (c.re, c.re, c.im, c.im))
        .reduce(|a, b| (a.0.min(b.0), a.1.max(b.1), a.2.min(b.2), a.3.max(b.3)))
        .unwrap();

    let sym_scale = Complex::new(sigma_max - sigma_min, omega_max - omega_min)*SYM_SIZE;

    let edges = (Complex::new(sigma_min, omega_min) - sym_scale*PADDING)..(Complex::new(sigma_max, omega_max) + sym_scale*PADDING);
    
    let area = BitMapBackend::new(plot_path, PLOT_RES).into_drawing_area();
    
    area.fill(&WHITE)?;
    
    let mut chart = ChartBuilder::on(&area)
        .caption(plot_title, PLOT_CAPTION_FONT.into_font())
        .margin(PLOT_MARGIN)
        .x_label_area_size(PLOT_LABEL_AREA_SIZE)
        .y_label_area_size(PLOT_LABEL_AREA_SIZE)
        .build_cartesian_2d(edges.start.re..edges.end.re, edges.start.im..edges.end.im)?;
    
    chart.configure_mesh()
        .set_all_tick_mark_size(0.1)
        .draw()?;
    
    for series in zeros.into_iter()
        .map(|pos| (pos, zero_sym.as_slice(), &BLUE))
        .chain(poles.into_iter()
            .flat_map(|pos| POLE_SYM.iter()
                .map(move |sym| (pos, sym.as_slice(), &RED))
            )
        ).map(|(pos, sym_stroke, color)| (
            sym_stroke.into_iter()
                .map(move |point| Complex::new(point.re*sym_scale.re, point.im*sym_scale.im) + pos)
                .collect::<Vec<Complex<f64>>>(),
            color
        )).chain(
            [
                (vec![Complex::new(edges.start.re, 0.0), Complex::new(edges.end.re, 0.0)], &BLACK),
                (vec![Complex::new(0.0, edges.start.im), Complex::new(0.0, edges.end.im)], &BLACK)
            ].into_iter()
        ).map(|(stroke, color)| LineSeries::new(
            stroke.into_iter().map(|point| (point.re, point.im)),
            color
        ))
    {
        chart.draw_series(series)?;
    }
        
    // To avoid the IO failure being ignored silently, we manually call the present function
    area.present().expect("Unable to write result to file");

    Ok(())
}

pub fn plot_bode<'a, F, const N: usize>(
    plot_title: &str, plot_path: &'a str,
    xy: [(F, Complex<F>); N],
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Float + AddAssign + SubAssign + 'static,
    Range<F>: AsRangedCoord<CoordDescType: ValueFormatter<<Range<F> as AsRangedCoord>::Value>, Value: Debug + Clone>,
    for<'b> &'b DynElement<'static, BitMapBackend<'a>, (F, F)>:
        PointCollection<'b, (
            <Range<F> as AsRangedCoord>::Value,
            <Range<F> as AsRangedCoord>::Value
        )>
{
    fn db<F>(x: F) -> F
    where
        F: Float
    {
        F::from(20.0).unwrap()*(x).log10()
    }

    let pi = F::from(PI).unwrap();
    let tau = F::from(TAU).unwrap();
    
    let mut y_arg_prev = F::zero();

    let xy = xy.map(|(x, y)| {
        let mut y_arg_offset = y.arg() - y_arg_prev;
        while y_arg_offset > pi
        {
            y_arg_offset -= tau;
        }
        while y_arg_offset < -pi
        {
            y_arg_offset += tau;
        }
        let y_arg = y_arg_offset + y_arg_prev;
        y_arg_prev = y_arg;
        (x, y.norm(), y_arg*F::from(180.0/PI).unwrap())
    });

    let ([x_min, y_norm_min, y_arg_min], [x_max, y_norm_max, y_arg_max]) = xy.into_iter()
        .map(|(x, y_norm, y_arg)| ([x, y_norm, y_arg], [x, y_norm, y_arg]))
        .reduce(|a, b| (a.0.zip(b.0).map(|(a, b)| a.min(b)), a.1.zip(b.1).map(|(a, b)| a.max(b))))
        .unwrap();
    
    let area = BitMapBackend::new(plot_path, PLOT_RES).into_drawing_area();
    
    area.fill(&WHITE)?;
    
    let mut chart_norm = ChartBuilder::on(&area)
        .caption(plot_title, PLOT_CAPTION_FONT.into_font())
        .margin(PLOT_MARGIN)
        .margin_bottom(PLOT_RES.1/2 + PLOT_MARGIN)
        .x_label_area_size(PLOT_LABEL_AREA_SIZE)
        .y_label_area_size(PLOT_LABEL_AREA_SIZE)
        .build_cartesian_2d(x_min..x_max, y_norm_min..y_norm_max)?;
    
    chart_norm.configure_mesh()
        .set_all_tick_mark_size(0.1)
        .draw()?;
    
    let xy_norm = xy.map(|(x, y_norm, _)| (x, y_norm));

    chart_norm.draw_series(LineSeries::new(
            xy_norm,
            &BLUE
        ))?;
        
    let mut chart_arg = ChartBuilder::on(&area)
        .margin(PLOT_MARGIN)
        .margin_top(PLOT_RES.1/2 + PLOT_MARGIN)
        .x_label_area_size(PLOT_LABEL_AREA_SIZE)
        .y_label_area_size(PLOT_LABEL_AREA_SIZE)
        .build_cartesian_2d(x_min..x_max, y_arg_min..y_arg_max)?;
    
    chart_arg.configure_mesh()
        .set_all_tick_mark_size(0.1)
        .draw()?;

    let xy_arg = xy.map(|(x, _, y_arg)| (x, y_arg));

    chart_arg.draw_series(LineSeries::new(
            xy_arg,
            &PURPLE
        ))?;
        
    // To avoid the IO failure being ignored silently, we manually call the present function
    area.present().expect("Unable to write result to file");

    Ok(())
}

pub fn plot_curve<'a, F, const N: usize>(
    plot_title: &str, plot_path: &'a str,
    xy: [(F, F); N],
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Float + 'static,
    Range<F>: AsRangedCoord<CoordDescType: ValueFormatter<<Range<F> as AsRangedCoord>::Value>, Value: Debug + Clone>,
    for<'b> &'b DynElement<'static, BitMapBackend<'a>, (F, F)>:
        PointCollection<'b, (
            <Range<F> as AsRangedCoord>::Value,
            <Range<F> as AsRangedCoord>::Value
        )>
{

    let ([x_min, y_min], [x_max, y_max]) = xy.into_iter()
        .map(|(x, y)| ([x, y], [x, y]))
        .reduce(|a, b| (a.0.zip(b.0).map(|(a, b)| a.min(b)), a.0.zip(b.0).map(|(a, b)| a.max(b))))
        .unwrap();
    
    let area = BitMapBackend::new(plot_path, PLOT_RES).into_drawing_area();
    
    area.fill(&WHITE)?;
    
    let mut chart = ChartBuilder::on(&area)
        .caption(plot_title, PLOT_CAPTION_FONT.into_font())
        .margin(PLOT_MARGIN)
        .x_label_area_size(PLOT_LABEL_AREA_SIZE)
        .y_label_area_size(PLOT_LABEL_AREA_SIZE)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;
    
    chart.configure_mesh()
        .set_all_tick_mark_size(0.1)
        .draw()?;
    
    chart.draw_series(LineSeries::new(
            xy,
            &BLUE
        ))?;
        
    // To avoid the IO failure being ignored silently, we manually call the present function
    area.present().expect("Unable to write result to file");

    Ok(())
}

pub fn plot_curve_2d<F, const NX: usize, const NY: usize>(
    plot_title: &str, plot_path: &str,
    x: [F; NX],
    y: [F; NY],
    f: impl Fn(F, F) -> F
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Float,
    Range<F>: AsRangedCoord<CoordDescType: ValueFormatter<<Range<F> as AsRangedCoord>::Value>, Value: Debug + Clone>,
    for<'b> &'b Polygon<(F, F, F)>:
        PointCollection<'b, (
            <Range<F> as AsRangedCoord>::Value,
            <Range<F> as AsRangedCoord>::Value,
            <Range<F> as AsRangedCoord>::Value
        )>
{
    use plotters::prelude::*;

    let area = SVGBackend::new(plot_path, PLOT_RES).into_drawing_area();
    
    let x_min = x.reduce(F::min).unwrap();
    let x_max = x.reduce(F::max).unwrap();
    
    let y_min = y.reduce(F::min).unwrap();
    let y_max = y.reduce(F::max).unwrap();

    let f_ref = &f;
    let f_values: Vec<F> = y.into_iter()
        .flat_map(|y| x.into_iter().map(move |x| f_ref(x, y)))
        .collect();

    let (z_min, z_max) = f_values.into_iter()
        .map(|f| (f, f))
        .reduce(|a, b| (a.0.min(b.0), a.1.max(b.1)))
        .unwrap();

    area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&area)
        .caption(plot_title, PLOT_CAPTION_FONT)
        .set_all_label_area_size(PLOT_LABEL_AREA_SIZE)
        .build_cartesian_3d(x_min..x_max, z_min..z_max, y_min..y_max)?;

    chart.with_projection(isometric);
    
    chart.configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .draw()?;
    
    chart.draw_series(
            SurfaceSeries::xoz(
                x.into_iter(),
                y.into_iter(),
                f,
            )
            .style(BLUE.mix(0.2).filled()),
        )?
        .label("Surface")
        .legend(|(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled()));
    
    chart.configure_series_labels()
        .border_style(&BLACK)
        .draw()?;
    
    // To avoid the IO failure being ignored silently, we manually call the present function
    area.present().expect("Unable to write result to file");

    Ok(())
}

pub fn plot_parametric_curve_2d<F, const NU: usize, const NV: usize>(
    plot_title: &str, plot_path: &str,
    u: [F; NU],
    v: [F; NV],
    f: impl Fn(F, F) -> [F; 3]
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Float + AddAssign,
    Range<F>: AsRangedCoord<CoordDescType: ValueFormatter<<Range<F> as AsRangedCoord>::Value>, Value: Debug + Clone>,
    for<'b> &'b Polygon<(F, F, F)>: PointCollection<'b, (
        <Range<F> as AsRangedCoord>::Value,
        <Range<F> as AsRangedCoord>::Value,
        <Range<F> as AsRangedCoord>::Value
    )>
{
    use plotters::prelude::*;

    let area = SVGBackend::new(plot_path, PLOT_RES).into_drawing_area();
    
    let f_ref = &f;
    let f_values: Vec<[F; 3]> = u.into_iter().flat_map(|u| v.into_iter().map(move |v| f_ref(u, v))).collect();

    let ([x_min, y_min, z_min], [x_max, y_max, z_max]) = f_values.into_iter()
        .map(|f| (f, f))
        .reduce(|a, b| (a.0.zip(b.0).map(|(a, b)| a.min(b)), a.1.zip(b.1).map(|(a, b)| a.max(b))))
        .unwrap();

    area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&area)
        .caption(plot_title, PLOT_CAPTION_FONT)
        .set_all_label_area_size(PLOT_LABEL_AREA_SIZE)
        .build_cartesian_3d(x_min..x_max, z_min..z_max, y_min..y_max)?;

    chart.with_projection(isometric);
    
    chart.configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .draw()?;
    
    chart.draw_series(
            SurfaceSeries::xoz(
                u.into_iter(),
                v.into_iter(),
                f,
            )
            .map(|polygon| {
                let mut exception = false;
                let mut sum_theta = F::zero();
                let mut n_theta = 0;
                let points: Vec<(F, F, F)> = polygon.point_iter()
                    .into_iter()
                    .map(|&(u, [x, y, z], v)| {
                        let theta = v.atan2(u);
                        if theta.is_finite()
                        {
                            sum_theta += theta;
                            n_theta += 1;
                        }
                        if v == F::zero() && u < F::zero()
                        {
                            exception = true;
                        }
                        (x, z, y)
                    })
                    .collect();
                let avg_theta = if exception {core::f64::consts::PI} else {sum_theta.to_f64().unwrap() / n_theta as f64};
                let c = (avg_theta/core::f64::consts::TAU + 1.0) % 1.0;
                Polygon::new(points, HSLColor(c, 1.0, 0.5).mix(0.2).filled())
            })
        )?
        .label("Surface")
        .legend(|(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled()));
    
    chart.configure_series_labels()
        .border_style(&BLACK)
        .draw()?;
    
    // To avoid the IO failure being ignored silently, we manually call the present function
    area.present().expect("Unable to write result to file");

    Ok(())
}

pub fn plot_curve_2d_rad<F, const NTHETA: usize, const NR: usize>(
    plot_title: &str, plot_path: &str,
    r: [F; NR],
    theta: [F; NTHETA],
    f: impl Fn(F, F) -> F
) -> Result<(), Box<dyn std::error::Error>>
where
    [(); 2*NR]:,
    F: Float + AddAssign,
    Range<F>: AsRangedCoord<CoordDescType: ValueFormatter<<Range<F> as AsRangedCoord>::Value>, Value: Debug + Clone>,
    for<'b> &'b Polygon<(F, F, F)>:
        PointCollection<'b, (
            <Range<F> as AsRangedCoord>::Value,
            <Range<F> as AsRangedCoord>::Value,
            <Range<F> as AsRangedCoord>::Value
        ), Point = &'b (
            F,
            F,
            F
        )>
{
    use plotters::prelude::*;

    let area = SVGBackend::new(plot_path, PLOT_RES).into_drawing_area();

    let r_max = r.into_iter()
        .map(|r| r.abs())
        .reduce(F::max)
        .unwrap();
    
    let (theta_min, theta_max) = theta.into_iter()
        .map(|theta| (theta, theta))
        .reduce(|a, b| (a.0.min(b.0), a.1.max(b.1)))
        .unwrap();

    let f_ref = &f;
    let f_values: Vec<F> = r.into_iter()
        .flat_map(|r| theta.into_iter()
            .map(move |theta| f_ref(r, theta))
        ).collect();
    
    let (z_min, z_max) = f_values.into_iter()
        .map(|f| (f, f))
        .reduce(|a, b| (a.0.min(b.0), a.1.max(b.1)))
        .unwrap();

    area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&area)
        .caption(plot_title, PLOT_CAPTION_FONT)
        .set_all_label_area_size(PLOT_LABEL_AREA_SIZE)
        .build_cartesian_3d(-r_max..r_max, z_min..z_max, -r_max..r_max)?;

    chart.with_projection(isometric);
    
    chart.configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .draw()?;

    chart.draw_series(
            SurfaceSeries::xoz(
                r.into_iter(),
                theta.into_iter(),
                |r, theta| f(r, theta),
            )
            //.style_func(&|&c| HSLColor(c as f64, 1.0, 0.5).mix(0.2).filled())
            .map(|polygon| {
                let mut sum_theta = F::zero();
                let points: Vec<(F, F, F)> = polygon.point_iter()
                    .into_iter()
                    .map(|&(r, z, theta)| {sum_theta += theta; (r*theta.cos(), z, r*theta.sin())})
                    .collect();
                let avg_theta = sum_theta / F::from(points.len()).unwrap();
                let c = (((avg_theta - theta_min)/(theta_max - theta_min)).to_f64().unwrap() + 1.0) % 1.0;
                Polygon::new(points, HSLColor(c, 1.0, 0.5).mix(0.2).filled())
            })
        )?
        .label("Radial surface")
        .legend(|(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled()));
    
    chart.configure_series_labels()
        .border_style(&BLACK)
        .draw()?;
    
    // To avoid the IO failure being ignored silently, we manually call the present function
    area.present().expect("Unable to write result to file");

    Ok(())
}

pub fn plot_parametric_curve_2d_rad<F, const NU: usize, const NV: usize>(
    plot_title: &str, plot_path: &str,
    u: [F; NU],
    v: [F; NV],
    f: impl Fn(F, F) -> [F; 3]
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Float + AddAssign,
    Range<F>: AsRangedCoord<CoordDescType: ValueFormatter<<Range<F> as AsRangedCoord>::Value>, Value: Debug + Clone>,
    for<'b> &'b Polygon<(F, F, F)>:
        PointCollection<'b, (
            <Range<F> as AsRangedCoord>::Value,
            <Range<F> as AsRangedCoord>::Value,
            <Range<F> as AsRangedCoord>::Value
        )>
{
    use plotters::prelude::*;

    let area = SVGBackend::new(plot_path, PLOT_RES).into_drawing_area();

    let f_ref = &f;
    let f_values: Vec<[F; 3]> = u.into_iter().flat_map(|u| v.into_iter().map(move |v| f_ref(u, v))).collect();

    let ([_r_min, theta_min, z_min], [r_max, theta_max, z_max]) = f_values.into_iter()
        .map(|f| (f, f))
        .reduce(|a, b| (a.0.zip(b.0).map(|(a, b)| a.min(b)), a.1.zip(b.1).map(|(a, b)| a.max(b))))
        .unwrap();

    area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&area)
        .caption(plot_title, PLOT_CAPTION_FONT)
        .set_all_label_area_size(PLOT_LABEL_AREA_SIZE)
        .build_cartesian_3d(-r_max..r_max, z_min..z_max, -r_max..r_max)?;

    chart.with_projection(isometric);
    
    chart.configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .draw()?;

    chart.draw_series(
            SurfaceSeries::xoz(
                u.into_iter(),
                v.into_iter(),
                |u, v| f(u, v),
            )
            //.style_func(&|&c| HSLColor(c as f64, 1.0, 0.5).mix(0.2).filled())
            .map(|polygon| {
                let mut sum_theta = F::zero();
                let points: Vec<(F, F, F)> = polygon.point_iter()
                    .into_iter()
                    .map(|&(_, [r, theta, z], _)| {sum_theta += theta; (r*theta.cos(), z, r*theta.sin())})
                    .collect();
                let avg_theta = sum_theta / F::from(points.len()).unwrap();
                let c = (((avg_theta - theta_min)/(theta_max - theta_min)).to_f64().unwrap() + 1.0) % 1.0;
                Polygon::new(points, HSLColor(c, 1.0, 0.5).mix(0.2).filled())
            })
        )?
        .label("Radial surface")
        .legend(|(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled()));
    
    chart.configure_series_labels()
        .border_style(&BLACK)
        .draw()?;
    
    // To avoid the IO failure being ignored silently, we manually call the present function
    area.present().expect("Unable to write result to file");

    Ok(())
}