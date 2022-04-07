use hound;
use clap::Parser;
use clap::ArgEnum;

#[derive(Parser)]
struct Args {
    #[clap(short, long, arg_enum, parse( try_from_str ))]
    target_khz: TargetKhz,
    #[clap(short, long, default_value = "tone")]
    file: String,
}

#[derive(Debug, Clone, Copy, ArgEnum, PartialEq, Eq, PartialOrd, Ord)]
enum TargetKhz {
    Two = 2,
    Four = 4,
    Six = 6,
    Eight = 8,
    Twelve = 12,
    Sixteen = 16,
    Thirtytwo = 32,
}

fn main() {
    let args = Args::parse();
    let mut sample = hound::WavReader::open(format!("assets/{}.wav", args.file)).unwrap();

    if sample.spec().channels != 1 {
        panic!("Audio is not mono!")
    }
    
    if sample.spec().bits_per_sample != 8 {
        panic!("BPS is not 8!")
    }

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: (args.target_khz as usize * 1000) as u32,
        bits_per_sample: sample.spec().bits_per_sample,
        sample_format: sample.spec().sample_format,
    };

    let sample_rate = sample.spec().sample_rate as usize;

    let mut writer = hound::WavWriter::create(format!("assets/{}_downsample.wav", args.file ), spec).unwrap();

    for (index, data) in sample.samples::<i8>().enumerate() {
        if index % ( sample_rate / 1000 ) / args.target_khz as usize == 0 {
            writer.write_sample(data.unwrap()).unwrap();
        }
    }
}
