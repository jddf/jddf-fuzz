use chrono::{DateTime, NaiveDateTime, Utc};
use clap::{App, AppSettings, Arg};
use failure::Error;
use jddf::schema::{Form, Type};
use jddf::{Schema, SerdeSchema};
use rand::seq::IteratorRandom;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;

fn main() -> Result<(), Error> {
    let matches = App::new("jsl-fuzz")
        .version("0.1")
        .about("Creates random JSON documents satisfying a JDDF schema")
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("n")
                .help("How many values to generate. Zero (0) indicates infinity")
                .default_value("0")
                .short("n")
                .long("num-values"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Where to read schema from. Dash (hypen) indicates stdin")
                .default_value("-"),
        )
        .get_matches();

    let num_values: usize = matches.value_of("n").unwrap().parse()?;

    let reader: Box<io::Read> = match matches.value_of("INPUT").unwrap() {
        "-" => Box::new(io::stdin()),
        file @ _ => Box::new(io::BufReader::new(File::open(file)?)),
    };

    let serde_schema: SerdeSchema = serde_json::from_reader(reader)?;
    let schema = Schema::from_serde(serde_schema)?;

    let mut rng = rand::thread_rng();
    let mut i = 0;
    while i != num_values || num_values == 0 {
        println!("{}", fuzz(&mut rng, &schema));
        i += 1;
    }

    Ok(())
}

fn fuzz<R: rand::Rng + ?Sized>(rng: &mut R, schema: &Schema) -> Value {
    match schema.form() {
        Form::Empty => fuzz_any(rng),
        Form::Type(Type::Boolean) => fuzz_bool(rng),
        Form::Type(Type::Int8) => fuzz_i8(rng),
        Form::Type(Type::Uint8) => fuzz_u8(rng),
        Form::Type(Type::Int16) => fuzz_i16(rng),
        Form::Type(Type::Uint16) => fuzz_u16(rng),
        Form::Type(Type::Int32) => fuzz_i32(rng),
        Form::Type(Type::Uint32) => fuzz_u32(rng),
        Form::Type(Type::Float32) => fuzz_f32(rng),
        Form::Type(Type::Float64) => fuzz_f64(rng),
        Form::Type(Type::String) => fuzz_string(rng),
        Form::Type(Type::Timestamp) => fuzz_timestamp(rng),
        Form::Enum(ref vals) => fuzz_enum(rng, vals),
        Form::Elements(ref sub_schema) => fuzz_elems(rng, sub_schema),
        Form::Properties {
            required,
            optional,
            allow_additional,
            ..
        } => fuzz_props(rng, required, optional, *allow_additional),
        Form::Values(ref sub_schema) => fuzz_values(rng, sub_schema),
        Form::Discriminator(ref tag, ref mapping) => fuzz_discr(rng, tag, mapping),
        _ => panic!(),
    }
}

fn fuzz_any<R: rand::Rng + ?Sized>(rng: &mut R) -> Value {
    vec![
        Value::Null,
        fuzz_bool(rng),
        fuzz_u8(rng),
        fuzz_f64(rng),
        fuzz_string(rng),
    ]
    .into_iter()
    .choose(rng)
    .unwrap()
}

fn fuzz_bool<R: rand::Rng + ?Sized>(rng: &mut R) -> Value {
    rng.gen::<bool>().into()
}

fn fuzz_i8<R: rand::Rng + ?Sized>(rng: &mut R) -> Value {
    rng.gen::<i8>().into()
}

fn fuzz_u8<R: rand::Rng + ?Sized>(rng: &mut R) -> Value {
    rng.gen::<u8>().into()
}

fn fuzz_i16<R: rand::Rng + ?Sized>(rng: &mut R) -> Value {
    rng.gen::<i16>().into()
}

fn fuzz_u16<R: rand::Rng + ?Sized>(rng: &mut R) -> Value {
    rng.gen::<u16>().into()
}

fn fuzz_i32<R: rand::Rng + ?Sized>(rng: &mut R) -> Value {
    rng.gen::<i32>().into()
}

fn fuzz_u32<R: rand::Rng + ?Sized>(rng: &mut R) -> Value {
    rng.gen::<u32>().into()
}

fn fuzz_f32<R: rand::Rng + ?Sized>(rng: &mut R) -> Value {
    rng.gen::<f32>().into()
}

fn fuzz_f64<R: rand::Rng + ?Sized>(rng: &mut R) -> Value {
    rng.gen::<f64>().into()
}

fn fuzz_str<R: rand::Rng + ?Sized>(rng: &mut R) -> String {
    (0..rng.gen_range(0, 8))
        .map(|_| rng.gen_range(32u8, 127u8) as char)
        .collect::<String>()
}

fn fuzz_string<R: rand::Rng + ?Sized>(rng: &mut R) -> Value {
    fuzz_str(rng).into()
}

fn fuzz_timestamp<R: rand::Rng + ?Sized>(rng: &mut R) -> Value {
    let date_time = NaiveDateTime::from_timestamp(rng.gen::<i32>() as i64, 0);
    let date_time = DateTime::<Utc>::from_utc(date_time, Utc);
    date_time.to_rfc3339().into()
}

fn fuzz_enum<R: rand::Rng + ?Sized>(rng: &mut R, vals: &HashSet<String>) -> Value {
    vals.iter().choose(rng).unwrap().clone().into()
}

fn fuzz_elems<R: rand::Rng + ?Sized>(rng: &mut R, sub_schema: &Schema) -> Value {
    (0..rng.gen_range(0, 8))
        .map(|_| fuzz(rng, sub_schema))
        .collect::<Vec<_>>()
        .into()
}

fn fuzz_props<R: rand::Rng + ?Sized>(
    rng: &mut R,
    required: &HashMap<String, Schema>,
    optional: &HashMap<String, Schema>,
    allow_additional: bool,
) -> Value {
    let mut vals = Vec::new();

    for (k, v) in required {
        vals.push((k.clone(), fuzz(rng, v)));
    }

    for (k, v) in optional {
        if rng.gen() {
            vals.push((k.clone(), fuzz(rng, v)));
        }
    }

    if allow_additional {
        for _ in 0..rng.gen_range(0, 8) {
            vals.push((fuzz_str(rng), fuzz_any(rng)));
        }
    }

    vals.into_iter()
        .collect::<serde_json::Map<String, Value>>()
        .into()
}

fn fuzz_values<R: rand::Rng + ?Sized>(rng: &mut R, sub_schema: &Schema) -> Value {
    (0..rng.gen_range(0, 8))
        .map(|_| {
            (
                fuzz_string(rng).as_str().unwrap().to_owned(),
                fuzz(rng, sub_schema),
            )
        })
        .collect::<serde_json::Map<String, Value>>()
        .into()
}

fn fuzz_discr<R: rand::Rng + ?Sized>(
    rng: &mut R,
    tag: &str,
    mapping: &HashMap<String, Schema>,
) -> Value {
    let (tag_val, sub_schema) = mapping.iter().choose(rng).unwrap();
    let mut obj = fuzz(rng, sub_schema);
    obj.as_object_mut()
        .unwrap()
        .insert(tag.to_owned(), tag_val.clone().into());
    obj
}
