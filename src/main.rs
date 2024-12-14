pub mod translations;
use calamine::{open_workbook_auto, Data, Range, Reader};
use std::env::args;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use translations::return_mapping;

use clap::Parser;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Source {
    Ingredients,
    Products,
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    solares: Option<String>,
    #[arg(short, long)]
    bpc: Option<String>,
    #[arg(short, long)]
    ingredientes: String,

    #[arg(short='x', long)]
    products_sheet: String,
    #[arg(short='y', long)]
    ingredients_sheet: String,
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[derive(Debug)]
enum Cat  {
    BPC,
    Solares
} 



fn main() {
    println!("running...");
    // converts first argument into a csv (same name, silently overrides
    // if the file already exists
    let clap_args = dbg!(Args::parse());
    let (file_productos, file_ingredientes, sheet_productos, sheet_ingredientes, source) = match clap_args.solares {
        Some(inner) => (inner, clap_args.ingredientes, clap_args.products_sheet, clap_args.ingredients_sheet, Cat::Solares ),
        None => match clap_args.bpc {
            Some(inner) => (inner, clap_args.ingredientes, clap_args.products_sheet, clap_args.ingredients_sheet, Cat::BPC),
            None => panic!("No se ha indicado ninguna referencia de productos")
            
        }
        
    };
    println!("{}", file_productos);
    let sce_prod = PathBuf::from(file_productos);
    let sce_ing = PathBuf::from(file_ingredientes);
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
        Cat::Solares =>     PathBuf::from("solares_productos_proc").with_extension("csv")
        
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

    let mut xl = open_workbook_auto(&sce_prod).unwrap();
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

fn write_range<W: Write>(
    dest: &mut W,
    range: Vec<Vec<&Data>>,
    source: Source,
) -> std::io::Result<()> {
    let delim = b'~';
    let translations = return_mapping(source);
    let mut writeable_headers = vec![];
    let mut table = vec![];
    for (n, r) in range.into_iter().enumerate() {
        let mut row_header = vec![];

        // Header
        if n == 0 {
            for (header_row_position, rowhead) in r.into_iter().enumerate() {
                match rowhead {
                    Data::String(s) => {
                        // Change the header names
                        let tra = translations.get(s);
                        match tra {
                            Some(header) => {
                                writeable_headers.push(header_row_position);
                                row_header.push(header.to_owned())
                            }
                            None => (),
                        }
                    }
                    _ => row_header.push("".to_owned()),
                }
            }
            table.push(row_header);
            // Other lines.
        } else {
            let mut row_body = vec![];
            for (body_row_position, c) in r.into_iter().enumerate() {
                if writeable_headers.contains(&body_row_position) {
                    let var_name = match *c {
                        Data::Empty => "".to_owned(),
                        Data::String(ref s) if s.contains("\"") => s.to_owned(),
                        Data::String(ref s)
                        | Data::DateTimeIso(ref s)
                        | Data::DurationIso(ref s) => s.to_owned(),

                        Data::Float(ref f) => format!("{}", f),
                        Data::DateTime(ref d) => format!("{}", d.as_f64()),
                        Data::Int(ref i) => format!("{}", i),
                        Data::Error(ref e) => {
                            // El error es que el archivo tiene #N/
                            format!("{:?}", f32::NAN)
                        }
                        Data::Bool(ref b) => format!("{}", b),
                    };
                    row_body.push(var_name)
                }
            }
            table.push(row_body)
        }
    }
    let mut writer = csv::WriterBuilder::new()
        .flexible(true)
        .delimiter(delim)
        .from_writer(dest);
    table
        .into_iter()
        .for_each(|row| writer.write_record(row).unwrap());
    Ok(())
}

fn process_product_files(range: &Range<Data>) -> (Vec<Vec<&Data>>, Vec<Vec<&Data>>) {
    let headers = range.headers().unwrap();
    let mut vec_ingredients = vec![];
    let mut vec_others = vec![];
    for r in range.rows() {
        let mut row_ingredients = vec![];
        let mut row_others = vec![];
        for (header, body) in headers.clone().into_iter().zip(r) {
            match header {
                h if h.eq("Descripcion") => {
                    row_ingredients.push(body);
                    row_others.push(body);
                }
                h if h.contains("Ingredient ") => row_ingredients.push(body),
                h if h.eq("") => (),
                _ => row_others.push(body),
            };
        }
        vec_ingredients.push(row_ingredients);
        vec_others.push(row_others);
    }
    return (vec_ingredients, vec_others);
}

fn process_ingredient_file(range: &Range<Data>) -> Vec<Vec<&Data>> {
    let mut vec_ingredients = vec![];
    for r in range.rows() {
        let mut rows = vec![];
        for datum in r {
            rows.push(datum)
        }
        vec_ingredients.push(rows)
    }
    return vec_ingredients;
}
