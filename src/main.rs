pub mod core;

use calamine::{open_workbook_auto, Reader};
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use core::translations::return_mapping;
use core::internals::{process_ingredient_file, process_product_files, write_range};

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
    command : Cat,
    #[arg(short, long)]
    productos: String,
    #[arg(short, long)]
    ingredientes: String,
    #[arg(short='x', long, default_value = "Productos")]
    products_sheet: String,
    #[arg(short='y', long, default_value = "Ingredientes_Formatted_V1")]
    ingredients_sheet: String,
}

#[derive(Debug, Clone, clap::Subcommand)] 
enum Cat {
    BPC,
    Solares,
    Home
}




fn main() {
    println!("running...");
    // converts first argument into a csv (same name, silently overrides
    // if the file already exists
    let clap_args = Args::parse();
    println!("{:?}", clap_args);
    let file_productos = clap_args.productos;
    let file_ingredientes = clap_args.ingredientes;
    let sheet_ingredientes = clap_args.ingredients_sheet;
    let sheet_productos = clap_args.products_sheet;
    let source = clap_args.command;
 




    let sce_prod = PathBuf::from(file_productos.trim());
    let sce_ing = PathBuf::from(file_ingredientes.trim());
    match sce_prod.extension().and_then(|s| s.to_str()) {
        Some("xlsx") | Some("xlsm") | Some("xlsb") | Some("xls") => (),
        _ => panic!("Expecting an excel file"),
    }
    match sce_ing.extension().and_then(|s| s.to_str()) {
        Some("xlsx") | Some("xlsm") | Some("xlsb") | Some("xls") => (),
        _ => panic!("Expecting an excel file"),
    }

    let dest_productos = match source {
        Cat::BPC =>     PathBuf::from("bpc_productos_proc").with_extension("csv"),
        Cat::Solares =>     PathBuf::from("solares_productos_proc").with_extension("csv"),
        Cat::Home => PathBuf::from("home_productos_proc").with_extension("csv")
        
    }; 


    let dest_ingredientes_productos =
        PathBuf::from("bpc_productos_proc_ingredientes").with_extension("csv");
    let dest_ingredientes = PathBuf::from("bpc_ingredientes_proc").with_extension("csv");
    println!(
        "running...\n{}\n{}\n{}\n",
        dest_productos.display(),
        dest_ingredientes.display(),
        dest_ingredientes_productos.display()
    );
    let mut dest_productos = BufWriter::new(File::create(dest_productos).unwrap());
    let mut dest_ingredientes_productos =
        BufWriter::new(File::create(dest_ingredientes_productos).unwrap());
    let mut dest_ingredientes = BufWriter::new(File::create(dest_ingredientes).unwrap());

    let mut xl = open_workbook_auto(&sce_prod).inspect_err(|e| {
        print!("Error parseando {}", e);
        let dir = sce_prod.ancestors().into_iter().map(|anc| anc.to_str().unwrap()).collect::<Vec<&str>>();
        eprintln!("Entre en el error en e2csv, con sce {}, con el directorio {:?} ", sce_prod.to_string_lossy(), dir )
    }).unwrap();
    let range = xl.worksheet_range(&sheet_productos).unwrap();
    let mut xl = open_workbook_auto(&sce_ing).unwrap();
    let range_ing = xl.worksheet_range(&sheet_ingredientes).unwrap();

    // write_range(&mut dest, &range).unwrap();
    let (productos_ingredientes, productos) = process_product_files(&range);
    let ingredientes = process_ingredient_file(&range_ing);
    let _ = write_range(&mut dest_productos, productos, Source::Products);
    let _ = write_range(
        &mut dest_ingredientes_productos,
        productos_ingredientes,
        Source::Products,
    );
    let _ = write_range(&mut dest_ingredientes, ingredientes, Source::Ingredients);
}

