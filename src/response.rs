use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
struct Wan {
    avgLatency: usize,
    downtime: usize,
    ispName: String,
    uptime: usize,
}

#[derive(Debug, Deserialize, Clone)]
struct Data2 {
    wan: Wan,
}

#[derive(Debug, Deserialize, Clone)]
struct Period {
    data: Data2,
    metricTime: String,
    version: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Data {
    hostId: String,
    metricType: String,
    siteId: String,
    periods: Vec<Period>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ResponseGetIspMetric {
    httpStatusCode: usize,
    traceId: String,
    data: Vec<Data>,
}

impl ResponseGetIspMetric {
    pub fn get_http_status_code(&self) -> usize {
        self.httpStatusCode
    }

    pub fn get_avg_latency(&self) -> Vec<(String, usize)> {
        let mut result = vec![];
        for value in self.data.clone() {
            for data in &value.periods {
                if data.data.wan.downtime > 0 {
                    result.push((data.metricTime.clone(), data.data.wan.avgLatency))
                }
            }
        }

        result
    }

    // pub fn get_avg_latency_v2(&self) -> Vec<(String, usize)> {
    //     let result = self.data.clone().iter().map(|val| {
    //             val.periods.pop()
    //         }).collect::<Vec<Period>>();

    //     self.data.it
    // .map(|val| {
    //     val.periods
    //         .iter()
    //         .map(|val2| (val2.metricTime, val2.data.wan.avgLatency))
    // })
    // .collect::<(String, usize)>();
    //     result
    // }
}
