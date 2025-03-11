#[cfg(test)]
pub mod tests{
	use calamine::{open_workbook_auto, Reader};
	use calamine::{Data, Range};

use crate::core::internals;
	#[test]
	pub fn parsero_ia() {
		let mut wb = calamine::open_workbook_auto("test_data/cortito.xlsx").unwrap();
		let range = wb.worksheet_range("cortito").unwrap();
		let result: Vec<Vec<Data>> = internals::process_food_ingredient_file(&range);
		for mut instance in result.clone() {
			instance.reverse();
			println!("{:?}", instance.first().unwrap())
		}
		assert_eq!(6, result.len())
	}
}