pub mod translations;
use calamine::{open_workbook_auto, Data, Range, Reader};
use std::{env, vec};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use translations::return_mapping;
#[derive(Debug)]
enum Source {
    Ingredients,
    Products
    
}



fn main() {
    // converts first argument into a csv (same name, silently overrides
    // if the file already exists
    let file_productos = "BPC_Productos (1).xlsx";
    let file_ingredientes = "BPC_Ingredientes.xlsx";
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

    let dest_productos = PathBuf::from("bpc_productos_proc").with_extension("csv");
    let dest_ingredientes_productos = PathBuf::from("bpc_productos_proc_ingredientes").with_extension("csv");
    let dest_ingredientes = PathBuf::from("bpc_ingredientes_proc").with_extension("csv");
    let mut dest_productos = BufWriter::new(File::create(dest_productos).unwrap());
    let mut dest_ingredientes_productos = BufWriter::new(File::create(dest_ingredientes_productos).unwrap());
    let mut dest_ingredientes = BufWriter::new(File::create(dest_ingredientes).unwrap());

    let mut xl = open_workbook_auto(&sce_prod).unwrap();
    let range = xl.worksheet_range("Productos").unwrap();
    let mut xl = open_workbook_auto(&sce_ing).unwrap();
    let range_ing = xl.worksheet_range("Ingredientes_Formatted_V1").unwrap();

    // write_range(&mut dest, &range).unwrap();
    let (productos_ingredientes, productos) = process_product_files(&range);
    let ingredientes = process_ingredient_file(&range_ing);
    let _ = write_range(&mut dest_productos, productos, Source::Products);
    let _ = write_range(&mut dest_ingredientes_productos, productos_ingredientes, Source::Products);
    let _ = write_range(&mut dest_ingredientes, ingredientes, Source::Ingredients);

}

fn write_range<W: Write>(dest: &mut W, range: Vec<Vec<&Data>>, source : Source) -> std::io::Result<()> {
    let translations = return_mapping(source);
    for (n,r) in range.into_iter().enumerate() {
        if n == 0 {
            write!(dest, ",")?;
            let limit = r.len();
            for (a, rowhead) in r.into_iter().enumerate() {
                match rowhead {
                    Data::String(s) => {
                        let tra = translations.get(s).unwrap();
                        write!(dest, "{}", tra).unwrap()
                    }
                    _ => write!(dest, "{}", "").unwrap()
                }
                if a < limit - 1 {
                    write!(dest, ",")?
                }
            }
            write!(dest, "\n")?;

        } else {
        let elem = format!("{},", n - 1 );
        write!(dest, "{}", &elem)?;

        for c in r.into_iter() {
            match *c {
                Data::Empty => Ok(()),
                Data::String(ref s) | Data::DateTimeIso(ref s) | Data::DurationIso(ref s) => {
                    write!(dest, "{}", s)
                }
                Data::Float(ref f) => write!(dest, "{}", f),
                Data::DateTime(ref d) => write!(dest, "{}", d.as_f64()),
                Data::Int(ref i) => write!(dest, "{}", i),
                Data::Error(ref e) => write!(dest, "{:?}", e),
                Data::Bool(ref b) => write!(dest, "{}", b),
            }?;
            write!(dest, ",")?
        }
        write!(dest, "\n")?;
    }
}
    Ok(())
}

fn process_product_files(range: &Range<Data>) -> (Vec<Vec<&Data>>, Vec<Vec<&Data>>) {
    let headers = range.headers().unwrap();
    let mut vec_ingredients = vec![];
    let mut vec_others = vec![];
    for (n,r) in range.rows().enumerate() {
            let mut row_ingredients = vec![];
            let mut row_others = vec![];
            if n == 0 {
                println!("{:?}", headers);
            }
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