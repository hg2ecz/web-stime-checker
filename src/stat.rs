// threadnum: 0, httpcode: Ok(200), pagesize: 21540, starttime: 0, elapsedtime: 185

#[derive(Clone, Debug)]
pub struct TimeStat {
    pub tmin: u128,
    pub tmax: u128,
    pub tsum: u128,
    pub tpcs: i32,
    pub terrpcs: i32,
}

pub fn min_max_time(data: &[super::meas::MeasData], maxsecond: usize) -> (Vec<TimeStat>, i32, i32) {
    let mut okcnt = 0;
    let mut errct = 0;
    let mut stat = vec![
        TimeStat {
            tmin: 0,
            tmax: 0,
            tsum: 0,
            tpcs: 0,
            terrpcs: 0
        };
        maxsecond
    ]; // max 4 hour
    for d in data {
        let idx = d.starttime as usize / 1000; // másodperc ... 0, 1, 2, 3
        if let Ok(x) = d.httpcode {
            if x == 200 {
                if stat[idx].tmin == 0 || stat[idx].tmin > d.elapsedtime {
                    stat[idx].tmin = d.elapsedtime;
                }
                if stat[idx].tmax < d.elapsedtime {
                    stat[idx].tmax = d.elapsedtime;
                }
                stat[idx].tsum += d.elapsedtime;
                stat[idx].tpcs += 1;
                okcnt += 1;
            } else {
                errct += 1;
                stat[idx].terrpcs += 1;
            }
        } else {
            errct += 1;
            stat[idx].terrpcs += 1;
        }
    }
    (stat, errct, okcnt)
}
