pub mod core;
pub mod test;
use core::internals::convert_files;
use core::translations::return_mapping;

use clap::Parser;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Source {
    Ingredients,
    Products,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Cat,
    #[arg(short, long)]
    productos: String,
    #[arg(short, long)]
    ingredientes: String,
    #[arg(short = 'x', long, default_value = "Productos")]
    products_sheet: String,
    #[arg(short = 'y', long, default_value = "Ingredientes_Formatted_V1")]
    ingredients_sheet: String,
}

#[derive(Debug, Clone, clap::Subcommand, Copy)]
enum Cat {
    BPC,
    Solares,
    Home,
    Foods,
}

fn main() {
    let clap_args = Args::parse();
    println!("Parsed Argumentsz\n{:?}", clap_args);
    let file_productos = clap_args.productos;
    let file_ingredientes = clap_args.ingredientes;
    let sheet_ingredientes = clap_args.ingredients_sheet;
    let sheet_productos = clap_args.products_sheet;
    let source = clap_args.command;
    convert_files(
        &file_productos,
        &file_ingredientes,
        &sheet_productos,
        &sheet_ingredientes,
        source,
    );
}
