use curl::easy::Easy;
use rand::Rng;
use std::thread;
use std::time::Instant;

#[derive(Debug)]
pub struct MeasData {
    pub threadnum: u32,
    pub httpcode: Result<u32, curl::Error>,
    pub pagesize: usize,
    pub starttime: u128,
    pub elapsedtime: u128,
}

fn get(url: &str) -> (Result<u32, curl::Error>, usize) {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(url).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }
    (handle.response_code(), data.len())
}

pub fn meas(url: &str, parallel: u32, maxseconds: u64, rand: bool) -> (Vec<usize>, Vec<MeasData>) {
    let mut children = vec![];
    for th in 0..parallel {
        let url = url.to_string();
        children.push(thread::spawn(move || {
            let mut stat = vec![];
            let mut rng = rand::thread_rng();
            let now = Instant::now();
            while now.elapsed().as_secs() < maxseconds {
                let realurl: String;
                if rand {
                    let randnum: u32 = rng.gen();
                    realurl = format!("{}/?tbench={}{}", url, 10000 + th, randnum);
                } else {
                    realurl = url.clone();
                }
                let get_start = now.elapsed().as_millis();
                let (respcode, dlen) = get(&realurl);
                let get_end = now.elapsed().as_millis();
                stat.push(MeasData {
                    threadnum: th,
                    httpcode: respcode,
                    pagesize: dlen,
                    starttime: get_start,
                    elapsedtime: get_end - get_start,
                });
            }
            stat
        }));
    }

    let mut reqperthread = vec![];
    let mut statdata = vec![];
    for ch in children {
        let d = ch.join().unwrap();
        reqperthread.push(d.len());
        statdata.extend(d);
    }
    //println!("{:?}", reqperthread);
    //println!("{:?}", statdata);
    (reqperthread, statdata)
}
