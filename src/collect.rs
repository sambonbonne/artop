use std::io;
use std::cmp::Ordering;

use systemstat::{System, Platform};
use systemstat::data::NetworkStats;
use num_cpus;

pub trait StatReader {
    fn new(system: System) -> Self;
    fn read(&self, scale: f64) -> io::Result<f64>;
}

fn calc_scale(value: f64, max: f64, scale: f64) -> f64 {
    (value / max) * scale
}

pub struct ProcessorLoadReader {
    system: System
}

impl StatReader for ProcessorLoadReader {
    fn new(system: System) -> ProcessorLoadReader {
        ProcessorLoadReader { system }
    }

    fn read(&self, scale: f64) -> io::Result<f64> {
        let load_average = self.system.load_average()?;
        Ok(calc_scale(load_average.one as f64, (num_cpus::get() + 1) as f64, scale))
    }
}

pub struct MemoryUsageReader {
    system: System
}

impl StatReader for MemoryUsageReader {
    fn new(system: System) -> MemoryUsageReader {
        MemoryUsageReader { system }
    }

    fn read(&self, scale: f64) -> io::Result<f64> {
        let memory = self.system.memory()?;
        let memory_total = memory.total.as_u64();
        let memory_usage = (&memory_total - memory.free.as_u64()) as f64;

        Ok(calc_scale(memory_usage, memory_total as f64, scale))
    }
}

pub struct NetworkUsageReader {
    system: System
}

impl StatReader for NetworkUsageReader {
    fn new(system: System) -> NetworkUsageReader {
        NetworkUsageReader { system }
    }

    fn read(&self, scale: f64) -> io::Result<f64> {
        let NetworkStats {
                rx_packets: rx,
                tx_packets: tx,
                ..
        } = self.system.network_stats("wlp1s0")?;

        let rx = rx as f64;
        let tx = tx as f64;

        let result = match rx.partial_cmp(&tx) {
            Some(Ordering::Less) => calc_scale(rx, tx, scale),
            Some(Ordering::Greater) => calc_scale(tx, rx, scale),
            Some(Ordering::Equal) => calc_scale(0.5, 1.0, scale),
            None => panic!("Could not compare network stats (rx/tx)")
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn processor_load() -> Result<(), String> {
        let processor_load_reader = ProcessorLoadReader::new(System::new());
        if let Ok(processor_load_scaled) = processor_load_reader.read(1000.0) {
            if processor_load_scaled >= 0.0 && processor_load_scaled <= 1000.0 {
                Ok(())
            } else {
                Err(String::from("Processor load reader should give a scaled value"))
            }
        } else {
            Err(String::from("Processor load reader should give an Ok Result"))
        }
    }

    #[test]
    fn memory_usage() -> Result<(), String> {
        let memory_usage_reader = MemoryUsageReader::new(System::new());
        if let Ok(memory_usage_scaled) = memory_usage_reader.read(1000.0) {
            if memory_usage_scaled >= 0.0 && memory_usage_scaled <= 1000.0 {
                Ok(())
            } else {
                Err(String::from("Memory usage reader should give a scaled value"))
            }
        } else {
            Err(String::from("Memory usage reader should give an Ok Result"))
        }
    }

    #[test]
    fn network_usage() -> Result<(), String> {
        let network_usage_reader = NetworkUsageReader::new(System::new());
        if let Ok(network_usage_scaled) = network_usage_reader.read(1000.0) {
            if network_usage_scaled >= 0.0 && network_usage_scaled <= 1000.0 {
                Ok(())
            } else {
                Err(String::from("Network usage reader should give a scaled value"))
            }
        } else {
            Err(String::from("Network usage reader should give an Ok Result"))
        }
    }}
