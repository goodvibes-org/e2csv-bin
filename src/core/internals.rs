use crate::{return_mapping, Cat, Source};
use calamine::{open_workbook_auto, DataType, Reader};
use calamine::{Data, Range};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;
use std::vec;

use super::translations::return_foods_mapping;

pub(crate) fn write_range<W: Write>(
    dest: &mut W,
    range: Vec<Vec<Data>>,
    source: Source,
    database: Cat,
) -> std::io::Result<()> {
    let delim = b'~';
    let translations = match database {
        Cat::BPC | Cat::Home | Cat::Solares => return_mapping(source),
        Cat::Foods => return_foods_mapping(source),
    };

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
                        let tra = translations.get(&s);
                        match tra {
                            Some(header) => {
                                writeable_headers.push(header_row_position);
                                row_header.push(header.to_owned())
                            }
                            None => (),
                        }
                    }
                    Data::Empty => {
                        writeable_headers.push(header_row_position);
                        row_header.push("droppable".to_owned());
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
                    let var_name = match c {
                        Data::Empty => "".to_owned(),
                        Data::String(ref s) if s.contains("\"") => s.to_owned(),
                        Data::String(ref s)
                        | Data::DateTimeIso(ref s)
                        | Data::DurationIso(ref s) => s.to_owned(),

                        Data::Float(ref f) => format!("{}", f),
                        Data::DateTime(ref d) => format!("{}", d.as_f64()),
                        Data::Int(ref i) => format!("{}", i),
                        Data::Error(ref _e) => {
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
pub(crate) fn process_food_ingredient_file(range: &Range<Data>) -> Vec<Vec<Data>> {
    let mut lens = HashSet::new();
    let mut same_ingredient: Vec<&[Data]> = vec![];
    let mut sinonimos_ingrediente = vec![];
    let mut data = vec![];
    for (n, instance) in range.rows().enumerate() {
        // Quedan los headers, salteo el ultimo
        let row = instance;
        if n.eq(&0) {
            let mut headers = vec![];
            headers.extend_from_slice(row);
            headers.push(Data::String("sinonimos".to_owned()));
            data.push(headers);
        } else if n.eq(&1) {
            // Por la estupidez de como esta armado el archivo
            continue;
        } else {
            if row.into_iter().all(|text| text.is_empty()) {
                let same_ingredient_clone = same_ingredient.clone();
                for row in same_ingredient.clone() {
                    sinonimos_ingrediente.push(row.get(0).unwrap().to_owned());
                }
                if let Some(ingrediente) = same_ingredient_clone.first() {
                    let mut vec_ing = vec![];
                    vec_ing.extend_from_slice(ingrediente);
                    //  bulk_create_sinonimo_ingrediente(conn, sinonimos_ingrediente);
                    let mut sinonimos_string = String::new();
                    for ingrediente in sinonimos_ingrediente.clone() {
                        sinonimos_string.push_str(&ingrediente.as_string().unwrap_or_default());
                        sinonimos_string.push(';');
                    }
                    let sinonimos_column = Data::String(sinonimos_string);
                    vec_ing.push(sinonimos_column);
                    lens.insert(vec_ing.len());
                    data.push(vec_ing);
                }
                same_ingredient.clear();
                sinonimos_ingrediente.clear();
            } else {
                same_ingredient.push(row);
            }
        }
    }
        return data;
}
pub(crate) fn process_product_files(range: &Range<Data>) -> (Vec<Vec<Data>>, Vec<Vec<Data>>) {
    let headers = range.headers().unwrap();
    let mut vec_ingredients = vec![];
    let mut vec_others = vec![];
    for r in range.rows() {
        let mut row_ingredients = vec![];
        let mut row_others = vec![];
        for (header, body) in headers.clone().into_iter().zip(r) {
            let body = body.clone();
            match header {
                h if h.eq("Descripcion") => {
                    row_ingredients.push(body.clone());
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

pub(crate) fn process_food_product_files(range: &Range<Data>) -> (Vec<Vec<Data>>, Vec<Vec<Data>>) {
    let headers = range.headers().unwrap();
    let mut vec_ingredients = vec![];
    let mut vec_others = vec![];
    for r in range.rows() {
        let mut row_ingredients = vec![];
        let mut row_others = vec![];
        for (header, body) in headers.clone().into_iter().zip(r) {
            let body = body.clone(); // No se bien como ese truquito me dio los datos que estaban
            match header {
                h if h.eq("Descripcion") => {
                    row_ingredients.push(body.clone());
                    row_others.push(body);
                }
                h if h.contains("Aditivo ") => row_ingredients.push(body),
                h if h.eq("") => (),
                _ => row_others.push(body),
            };
        }
        vec_ingredients.push(row_ingredients);
        vec_others.push(row_others);
    }

    return (vec_ingredients, vec_others);
}

pub(crate) fn process_ingredient_file(range: &Range<Data>) -> Vec<Vec<Data>> {
    let mut vec_ingredients = vec![];
    for r in range.rows() {
        let mut rows = vec![];
        rows.extend_from_slice(r);
        vec_ingredients.push(rows)
    }
    return vec_ingredients;
}
pub(crate) fn convert_files(
    file_productos: &str,
    file_ingredientes: &str,
    sheet_productos: &str,
    sheet_ingredientes: &str,
    source: Cat,
) {
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
        Cat::BPC => PathBuf::from("bpc_productos_proc").with_extension("csv"),
        Cat::Solares => PathBuf::from("solares_productos_proc").with_extension("csv"),
        Cat::Home => PathBuf::from("home_productos_proc").with_extension("csv"),
        Cat::Foods => PathBuf::from("foods_productos_proc").with_extension("csv"),
    };

    let dest_ingredientes_productos = match source {
        Cat::BPC => PathBuf::from("bpc_productos_proc_ingredientes").with_extension("csv"),
        Cat::Home => PathBuf::from("home_productos_proc_ingredientes").with_extension("csv"),
        Cat::Solares => PathBuf::from("solares_productos_proc_ingredientes").with_extension("csv"),
        Cat::Foods => PathBuf::from("foods_productos_proc_ingredientes").with_extension("csv"),
    };
    let dest_ingredientes = match source {
        Cat::BPC => PathBuf::from("bpc_ingredientes_proc").with_extension("csv"),
        Cat::Solares => PathBuf::from("solares_ingredientes_proc").with_extension("csv"),
        Cat::Home => PathBuf::from("home_ingredientes_proc").with_extension("csv"),
        Cat::Foods => PathBuf::from("foods_ingredientes_proc").with_extension("csv"),
    };
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

    let mut xl = open_workbook_auto(&sce_prod)
        .inspect_err(|e| {
            print!("Error parseando {}", e);
            let dir = sce_prod
                .ancestors()
                .into_iter()
                .map(|anc| anc.to_str().unwrap())
                .collect::<Vec<&str>>();
            eprintln!(
                "Entre en el error en e2csv, con sce {}, con el directorio {:?} ",
                sce_prod.to_string_lossy(),
                dir
            )
        })
        .unwrap();
    let range = xl.worksheet_range(&sheet_productos).unwrap();
    let mut xl = open_workbook_auto(&sce_ing).unwrap();
    let range_ing = xl.worksheet_range(&sheet_ingredientes).unwrap();

    // write_range(&mut dest, &range).unwrap();
    let (productos_ingredientes, productos) = match source {
        Cat::BPC | Cat::Home | Cat::Solares => process_product_files(&range),
        Cat::Foods => process_food_product_files(&range),
    };

    let ingredientes = match source {
        Cat::BPC | Cat::Home | Cat::Solares => process_ingredient_file(&range_ing),
        Cat::Foods => process_food_ingredient_file(&range_ing).into(),
    };
    let _ = write_range(&mut dest_productos, productos, Source::Products, source);
    let _ = write_range(
        &mut dest_ingredientes_productos,
        productos_ingredientes,
        Source::Products,
        source,
    );
    let _ = write_range(
        &mut dest_ingredientes,
        ingredientes,
        Source::Ingredients,
        source,
    );
}
