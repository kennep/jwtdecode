use structopt::StructOpt;
use anyhow::{Context, Result, bail};
use std::io::{Read};

#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read the JWT from
    #[structopt(default_value = "-", parse(from_os_str))]
    path: std::path::PathBuf,
}

fn read_to_string<R: Read>(reader: &mut R) -> std::io::Result<String>
{
    let mut buffer = String::new();
    return match reader.read_to_string(&mut buffer)
    {
        Ok(_) => Ok(buffer),
        Err(e) => Err(e)
    };
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let path = args.path;
    let mut content = 
        match path.to_str() {
            Some("-") => read_to_string(&mut std::io::stdin())
                .with_context(|| "could not read from standard input"),
            _ => std::fs::read_to_string(&path)
                .with_context(|| format!("could not read file `{}`", path.display()))
        }?;

    content.retain(|c| !c.is_whitespace());
    let items: Vec<&str> = content.split(".").collect();

    if items.len() != 3
    {
        bail!("Invalid JWT: Expected three items separated by string");
    }

    let header = items[0];
    let payload = items[1];
    let signature = items[2];

    let header_dec = base64::decode_config(header, base64::URL_SAFE_NO_PAD)
        .with_context(|| "Header is not valid base64 url-safe encoded string")?;
    let payload_dec = base64::decode_config(payload, base64::URL_SAFE_NO_PAD)
        .with_context(|| "Payload is not valid base64 url-safe encoded string")?;

    let header_val:serde_json::Value = serde_json::from_slice(&header_dec)
        .with_context(|| "Header is not valid JSON")?;
    let header_str = serde_json::to_string_pretty(&header_val)?;

    let payload_val:serde_json::Value = serde_json::from_slice(&payload_dec)
        .with_context(|| "Payload is not valid JSON")?;
    let payload_str = serde_json::to_string_pretty(&payload_val)?;

    println!("Header: {}", header_str);
    println!("Payload: {}", payload_str);
    println!("Signature: {}", signature);
    Ok(())
}
