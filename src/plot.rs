use plotters::prelude::*;

pub fn plot_min_max_time(
    fname: &str,
    tstat: Vec<super::stat::TimeStat>,
    url: &str,
    threads: u32,
    rand_text: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut maxval = tstat
        .iter()
        .max_by(|x, y| x.tmax.cmp(&y.tmax))
        .unwrap()
        .tmax;
    let maxval_tpcs = tstat
        .iter()
        .max_by(|x, y| x.tpcs.cmp(&y.tpcs))
        .unwrap()
        .tpcs as u128;
    // let abssum: f64 = tstat.iter().map(|x| x.tsum as f64).sum();
    let abscnt: f64 = tstat.iter().map(|x| x.tpcs as f64).sum();
    //    let pcsmultiply = tstat.len() as f64*abssum/abscnt/abscnt * 3.;
    let pcsmultiply: i32 = 10; // tízszerese a kiszolgálás/sec-nek
    if maxval_tpcs * pcsmultiply as u128 > maxval {
        maxval = maxval_tpcs * pcsmultiply as u128;
    };

    let root = BitMapBackend::new(fname, (1870, 1000)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!(
                "{} és {} szálon {} kiszolgálva: {} ({}/sec)",
                url,
                threads,
                rand_text,
                abscnt,
                abscnt as usize / tstat.len()
            ),
            ("Arial", 50).into_font(),
        )
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0..tstat.len(), 0..maxval as i32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(
            AreaSeries::new(
                (0..).zip(tstat.iter().map(|x| x.tmax as i32)), // max
                0,                                              // min
                &RED.mix(0.2),                                  // Make the series opac
            )
            .border_style(&RED), // Make a brighter border
        )?
        .label("Maximum");

    chart
        .draw_series(LineSeries::new(
            (0..).zip(tstat.iter().map(|x| {
                if x.tpcs > 0 {
                    x.tsum as i32 / x.tpcs
                } else {
                    0
                }
            })),
            &GREEN,
        ))?
        .label("Median");

    chart
        .draw_series(LineSeries::new(
            (0..).zip(tstat.iter().map(|x| x.tmin as i32)),
            &BLUE,
        ))?
        .label("Minimum");

    // kiszolgálásszám
    chart
        .draw_series(LineSeries::new(
            (0..).zip(tstat.iter().map(|x| x.tpcs * pcsmultiply)),
            &BLACK,
        ))?
        .label("Kiszolgálásszám");

    // kiszolgálásszám ... hibás
    chart
        .draw_series(LineSeries::new(
            (0..).zip(tstat.iter().map(|x| x.terrpcs * pcsmultiply)),
            &BLACK,
        ))?
        .label("Kiszolgálásszám");

    Ok(())
}
