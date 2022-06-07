use structopt::StructOpt;
use systemstat::{Platform, System};

mod collect;
mod convert;
mod write;

use collect::StatReader;
use convert::Converter;
use write::{fillers, Filler};

const DEFAULT_ROUNDS: u32 = 1000;
const DEFAULT_INTERVAL: u64 = 500;

#[derive(StructOpt)]
struct ArtopCli {
    #[structopt(
        short = "f",
        long = "fill-method",
        env = "ARTOP_FILL_METHOD",
        help = "The image filling method"
    )]
    fill_method: String,
    #[structopt(
        short = "o",
        long = "output",
        env = "ARTOP_OUTPUT",
        parse(from_os_str),
        help = "The image output path"
    )]
    output_path: std::path::PathBuf,
}

#[derive(Debug)]
struct ArtopError(String);

fn main() -> Result<(), ArtopError> {
    let args = ArtopCli::from_args();

    let output_path = args
        .output_path
        .into_os_string()
        .into_string()
        .map_err(|_err| ArtopError(String::from("Given output is not a valid path ({})")))?;

    let processor_load_reader = collect::ProcessorLoadReader::new(System::new());
    let memory_usage_reader = collect::MemoryUsageReader::new(System::new());
    let network_usage_reader = collect::NetworkUsageReader::new(System::new());

    let mut converter: Converter<
        collect::ProcessorLoadReader,
        collect::MemoryUsageReader,
        collect::NetworkUsageReader,
    > = Converter::<
        collect::ProcessorLoadReader,
        collect::MemoryUsageReader,
        collect::NetworkUsageReader,
    >::new(
        &processor_load_reader,
        &memory_usage_reader,
        &network_usage_reader,
    );

    let filler = match get_filler(&args.fill_method) {
        Some(filler) => filler,
        None => {
            return Err(ArtopError(format!(
                "Given fill method \"{}\" does not exist",
                args.fill_method
            )))
        }
    };

    let converter_error = match converter.run(DEFAULT_ROUNDS, DEFAULT_INTERVAL) {
        Ok(true) => {
            converter.finish(filler, output_path);
            return Ok(());
        }
        Ok(false) => ArtopError(String::from(
            "The converter failed but did not report any error",
        )),
        Err(e) => ArtopError(format!("The converter reported an error: {}", e)),
    };

    Err(converter_error)
}

fn get_filler(method: &str) -> Option<Filler> {
    match method {
        "smooth" => Some(fillers::smooth),
        "gradient" => Some(fillers::gradient),
        _ => None,
    }
}
