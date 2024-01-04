use clap::Parser;
use anchor_idl::Idl;
use heck::{ ToLowerCamelCase, ToUpperCamelCase };
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    /// The path to the json file to convert.
    path: std::path::PathBuf,
    /// The directory to save the output file. Defaults to dir of json file.
    #[arg(short, long)]
    outdir: Option<std::path::PathBuf>,
}

pub fn idl_ts(idl: &Idl) -> Result<String, std::io::Error> {
    let mut idl = idl.clone();
    for acc in idl.accounts.iter_mut() {
        acc.name = acc.name.to_lower_camel_case();
    }
    let idl_json = serde_json::to_string_pretty(&idl)?;
    Ok(format!(
        r#"export type {} = {};

export const IDL: {} = {};
"#,
        idl.name.to_upper_camel_case(),
        idl_json,
        idl.name.to_upper_camel_case(),
        idl_json
    ))
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let bytes = std::fs::read(&args.path).with_context(|| format!("could not read file `{}`", args.path.display()))?;
    let idl: Idl = serde_json::from_reader(&*bytes).expect("Invalid IDL format.");
    let ts_idl = idl_ts(&idl)?;
    
    let mut default_path = args.path;
    default_path.pop();

    let out = match args.outdir {
        None => default_path,
        Some(path) => path,
    };

    let ts_out = std::path::PathBuf::from(out.join(&idl.name).with_extension("ts"));

    std::fs::write(&ts_out, ts_idl)?;
    println!("file created at: {}", ts_out.display());
    Ok(())
}

