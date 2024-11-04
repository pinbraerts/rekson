use clap::Parser;
use rand::distributions::Alphanumeric;
use rand::{Rng, RngCore, SeedableRng};

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum ValueType {
    String,
    Number,
    Object,
    Array,
    Bool,
    Null,
}

fn generate_type(depth: usize, random: &mut impl RngCore) -> ValueType {
    let choose = if depth == 0 {
        vec![
            ValueType::String,
            ValueType::Number,
            ValueType::Bool,
            ValueType::Null,
        ]
    } else {
        vec![
            ValueType::String,
            ValueType::Number,
            ValueType::Object,
            ValueType::Array,
            ValueType::Bool,
            ValueType::Null,
        ]
    };
    choose[random.gen_range(0..choose.len())]
}

fn generate_value(length: usize, depth: usize, random: &mut impl RngCore) -> String {
    let value_type = generate_type(depth, random);
    generate(length, depth, value_type, random)
}

fn generate(
    length: usize,
    depth: usize,
    value_type: ValueType,
    random: &mut impl RngCore,
) -> String {
    match value_type {
        ValueType::String => {
            let len = random.gen_range((if depth == 0 { 0 } else { 1 })..length);
            let string: String = random
                .sample_iter(Alphanumeric)
                .take(len)
                .map(|c| c as char)
                .collect();
            format!("\"{string}\"")
        }
        ValueType::Number => random.gen::<i32>().to_string(),
        ValueType::Object => {
            let len = random.gen_range(0..length);
            let content = (0..len)
                .map(|_| {
                    let key = generate(length, 1, ValueType::String, random);
                    let value = generate_value(length, depth - 1, random);
                    format!("{key}:{value}")
                })
                .collect::<Vec<_>>()
                .join(",");
            format!("{{{content}}}")
        }
        ValueType::Array => {
            let len = random.gen_range(0..length);
            let content = (0..len)
                .map(|_| generate_value(length, depth - 1, random))
                .collect::<Vec<String>>()
                .join(", ");
            format!("[{content}]")
        }
        ValueType::Bool => random.gen::<bool>().to_string(),
        ValueType::Null => "null".into(),
    }
}

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    pub seed: i32,

    #[arg(short = 't', long = "type", value_enum, default_value_t = ValueType::Object)]
    pub value_type: ValueType,

    #[arg(short = 'd', long = "depth", default_value_t = 10)]
    pub max_depth: usize,

    #[arg(short = 'l', long = "length", default_value_t = 10)]
    pub max_length: usize,
}

fn main() {
    let args = Args::parse();
    let mut seed = [0u8; 32];
    seed.iter_mut()
        .enumerate()
        .for_each(|(i, v)| *v = ((args.seed >> i) & 0xff) as u8);
    let seed = seed;
    let mut random = rand::rngs::SmallRng::from_seed(seed);
    let value = generate(
        args.max_length,
        args.max_depth,
        args.value_type,
        &mut random,
    );
    println!("{value}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let mut random = rand::rngs::SmallRng::from_entropy();
        let value = generate(10, 10, ValueType::Object, &mut random);
        print!("{value}");
        json::parse(&value).expect("valid json");
    }
}
