use anyhow::{bail, Context, Result};
use std::io::Read;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read the JWT from
    #[structopt(default_value = "-", parse(from_os_str))]
    path: std::path::PathBuf,

    /// Select a field by JSON path
    #[structopt(long = "--jsonpath", short = "-p")]
    jsonpath: Option<String>,

    /// Output raw field value (when used with --jsonpath to select
    /// a single field)
    #[structopt(long = "--raw", short = "-r")]
    raw: bool,
}

fn read_to_string<R: Read>(reader: &mut R) -> std::io::Result<String> {
    let mut buffer = String::new();
    match reader.read_to_string(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(e) => Err(e),
    }
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let path = args.path;
    let mut content = match path.to_str() {
        Some("-") => read_to_string(&mut std::io::stdin())
            .with_context(|| "could not read from standard input"),
        _ => std::fs::read_to_string(&path)
            .with_context(|| format!("could not read file `{}`", path.display())),
    }?;

    content.retain(|c| !c.is_whitespace());
    let items: Vec<&str> = content.split('.').collect();

    if items.len() != 3 {
        bail!("Invalid JWT: Expected three items separated by string");
    }

    let header = items[0];
    let payload = items[1];
    let signature = items[2];

    let header_dec = base64::decode_config(header, base64::URL_SAFE_NO_PAD)
        .with_context(|| "Header is not valid base64 url-safe encoded string")?;
    let payload_dec = base64::decode_config(payload, base64::URL_SAFE_NO_PAD)
        .with_context(|| "Payload is not valid base64 url-safe encoded string")?;

    let header_val: serde_json::Value =
        serde_json::from_slice(&header_dec).with_context(|| "Header is not valid JSON")?;

    let payload_val: serde_json::Value =
        serde_json::from_slice(&payload_dec).with_context(|| "Payload is not valid JSON")?;

    let output_val = serde_json::json!({
        "header": header_val,
        "payload": payload_val,
        "signature": signature
    });
    let output_val = match args.jsonpath {
        Some(path) => match jsonpath_lib::select(&output_val, &path)
            .with_context(|| "Invalid jsonpath")?[..]
        {
            [a, ..] => a,
            [] => bail!(
                "No match for {} in {}",
                path,
                serde_json::to_string_pretty(&output_val)?
            ),
        },
        None => &output_val,
    };

    if args.raw {
        if let Some(val) = output_val.as_str() {
            println!("{}", val);
        }
    }

    let output_str = serde_json::to_string_pretty(output_val)?;
    println!("{}", output_str);
    Ok(())
}
