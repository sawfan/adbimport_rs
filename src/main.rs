use std::path::PathBuf;

use adbimport::{
    parse_astrodatabank_export_file, parse_astrodatabank_export_file_structured, write_gedcom7_file,
};

use kleio::{
    archive_genealogy_archive,
    save_genealogy_index_archive,
    GenealogyIndex,
};
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Gedcom7,
    Rkyv,
}

#[derive(Debug, Parser)]
#[command(
    name = "adbimport",
    version,
    about = "Convert Astrodatabank XML export to GEDCOM 7 (or emit a structured rkyv archive)"
)]
struct Args {
    /// Input Astrodatabank XML export file
    input_xml: PathBuf,

    /// Output file path (either .ged or .rkyv depending on --format)
    output: PathBuf,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Gedcom7)]
    format: OutputFormat,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.format {
        OutputFormat::Gedcom7 => {
            let export = parse_astrodatabank_export_file(&args.input_xml)?;
            write_gedcom7_file(&export, &args.output)?;
        }
        OutputFormat::Rkyv => {
            let (_export, structured) =
                parse_astrodatabank_export_file_structured(&args.input_xml)?;

            // Build the runtime index and convert to an archivable snapshot.
            let index = GenealogyIndex::build(
                structured.people,
                structured.events,
                structured.families,
                structured.places,
                structured.notes,
            );
            let archive = index.to_archive();

            let bytes = archive_genealogy_archive(&archive)?;
            save_genealogy_index_archive(&args.output, &bytes)?;
        }
    }

    Ok(())
}
