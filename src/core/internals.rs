use calamine::{Data, Range};
use std::io::Write;
use crate::Source;
use crate::return_mapping;


pub(crate)  fn write_range<W: Write>(
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
    
    pub(crate)  fn process_product_files(range: &Range<Data>) -> (Vec<Vec<&Data>>, Vec<Vec<&Data>>) {
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
    
   pub(crate)  fn process_ingredient_file(range: &Range<Data>) -> Vec<Vec<&Data>> {
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
    