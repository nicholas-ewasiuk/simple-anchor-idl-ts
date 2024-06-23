use anchor_idl::Idl;
use anyhow::{Context, Result};
use clap::Parser;
use heck::{ToLowerCamelCase, ToUpperCamelCase};

#[derive(Parser)]
struct Cli {
    /// The path to the json file to convert.
    path: std::path::PathBuf,

    /// The directory to save the output file. Defaults to dir of json file.
    #[clap(short, long)]
    outdir: Option<std::path::PathBuf>,

    /// Name of the IDL type and exported const to generate. Defaults to the self-defined IDL name.
    #[clap(short = 'n', long)]
    program_name: Option<String>,
}

pub fn idl_ts(idl: &mut Idl, name_arg: Option<String>) -> Result<String, std::io::Error> {
    for acc in idl.accounts.iter_mut() {
        acc.name = acc.name.to_lower_camel_case();
    }
    let idl_json = serde_json::to_string_pretty(&idl)?;

    let idl_name = name_arg.unwrap_or_else(|| idl.name.to_upper_camel_case());

    let type_name = format!("{idl_name}IDLType");
    let idl_const_name = format!("{idl_name}IDL");

    let type_export = format!("export type {type_name} = {idl_json};");
    let const_export = format!("export const {idl_const_name}: {type_name} = {idl_json};");

    Ok(format!("{type_export}\n\n{const_export}\n"))
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let bytes = std::fs::read(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    let mut idl: Idl = serde_json::from_slice(&bytes).expect("Invalid IDL format.");
    let ts_idl = idl_ts(&mut idl, args.program_name)?;

    let mut default_path = args.path;
    default_path.pop();

    let out = match args.outdir {
        None => default_path,
        Some(path) => path,
    };

    let ts_out = out.join(&idl.name).with_extension("ts");

    std::fs::write(&ts_out, ts_idl)?;
    println!("file created at: {}", ts_out.display());
    Ok(())
}
