pub mod ch0801;
pub mod ch0802;
pub mod ch0803;
pub mod ch0901;
pub mod ch0902;
pub mod ch1001;
pub mod ch1002;
pub mod ch1003;
pub mod ch1101;
pub mod ch1102;
pub mod ch1301;
pub mod ch1302;
pub mod ch1303;
pub mod ch1501;
pub mod ch1502;
pub mod ch1503;
pub mod ch1504;
pub mod ch1505;
pub mod ch1506;
pub mod ch16;
pub mod ch17;
pub mod ch18;

pub mod ch1103;
#[cfg(test)]
mod ch1103_integration_tests;

pub fn run() {
    ch0801::run();
    ch0802::run();
    ch0803::run();
    ch0901::run();
    ch0902::run();
    ch1001::run();
    ch1002::run();
    ch1003::run();
    ch1101::run();
    ch1102::run();
    ch1103::run();
    ch1301::run();
    ch1302::run();
    ch1303::run();
    ch1501::run();
    ch1502::run();
    ch1503::run();
    ch1504::run();
    ch1505::run();
    ch1506::run();
    ch16::run();
    ch17::run();
    ch18::run();
}
