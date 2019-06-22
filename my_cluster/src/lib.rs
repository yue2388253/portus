use std::collections::HashMap;
use portus::ipc::Ipc;
use portus::{Slave, Aggregator, CongAlg, Config, Datapath, DatapathInfo, DatapathTrait, Report};
use cluster_message_types::{summary::Summary, allocation::Allocation};

pub struct ClusterExample<T: Ipc> {
    sub_flows: HashMap<u32, SubFlow<T>>,
}

pub struct SubFlow<T: Ipc> {
    control: Datapath<T>,
}

#[derive(Clone)]
pub struct ClusterExampleConfig {

}

impl Default for ClusterExampleConfig {
    fn default() -> Self {
        ClusterExampleConfig {
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
#[derive(Clone, Copy, Debug)]
pub struct BottleneckID(u32);
impl From<DatapathInfo> for BottleneckID {
    // TODO Just uses source IP address for now, but should eventually use a
    // better method of determining which flows share the same bottleneck.
    fn from(d: DatapathInfo) -> Self {
        BottleneckID(d.src_ip)
    }
}

impl Default for BottleneckID {
    fn default() -> BottleneckID { BottleneckID(0) }
}

impl<T: Ipc> Aggregator<T> for ClusterExample<T> {
    type Key = BottleneckID;

    fn new_flow(&mut self, control: Datapath<T>, info: DatapathInfo) {

    }

    fn close_one(&mut self, sock_id: u32) {

    }
}

impl<T: Ipc> CongAlg<T> for ClusterExample<T> {
    type Config = ClusterExampleConfig;

    fn name() -> String {
        String::from("cluster-aggregation")
    }

    fn init_programs() -> Vec<(String, String)> {
        vec![(
            String::from("default"), String::from("
                (def (Report
                    (volatile acked 0)
                    (volatile sacked 0)
                    (volatile loss 0)
                    (volatile timeout false)
                    (volatile rtt 0)
                    (volatile inflight 0)
                    (volatile pending 0)
                ))
                (when true
                    (:= Report.inflight Flow.packets_in_flight)
                    (:= Report.rtt Flow.rtt_sample_us)
                    (:= Report.acked (+ Report.acked Ack.bytes_acked))
                    (:= Report.sacked (+ Report.sacked Ack.packets_misordered))
                    (:= Report.loss Ack.lost_pkts_sample)
                    (:= Report.timeout Flow.was_timeout)
                    (:= Report.pending Flow.bytes_pending)
                    (fallthrough)
                )
                (when (|| Report.timeout (> Report.loss 0))
                    (report)
                    (:= Micros 0)
                )
                (when (> Micros Flow.rtt_sample_us)
                    (report)
                    (:= Micros 0)
                )
            ")
        )]
    }

    fn create(control: Datapath<T>, cfg: Config<T, ClusterExample<T>>, info: DatapathInfo) -> Self {
        let s = Self {
            sub_flows: HashMap::new(),
        };
        s
    }

    fn on_report(&mut self, sock_id: u32, r: Report) {
//        let _ = Slave::on_report(self, sock_id, r);
    }

}

impl<T: Ipc> Slave for ClusterExample<T> {
    fn create_summary(&mut self) -> Option<&Summary> {
        None
    }

    fn next_summary_time(&mut self) -> u32 {
        // max(self.min_rtt, 25_000)
				25_000
    }

    fn on_allocation(&mut self, a: &Allocation) {
    }


    fn on_report(&mut self, sock_id: u32, r: Report) -> bool {
        true
    }


}