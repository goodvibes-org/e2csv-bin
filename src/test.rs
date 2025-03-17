#[cfg(test)]
pub mod tests {
    use calamine::Data;
    use calamine::{open_workbook_auto, Reader};

    use crate::core::internals;
    #[test]
    pub fn correctly_identifies_foods_ingredient_number() {
        let mut wb = open_workbook_auto("test_data/cortito.xlsx").unwrap();
        let range = wb.worksheet_range("cortito").unwrap();
        let result: Vec<Vec<Data>> = internals::process_food_ingredient_file(&range);
        println!("[DEBUG] Header row: {:?}", result.clone().first().unwrap());
        println!("{:#?}", result.clone().get(4).unwrap());
        for mut instance in result.clone() {
            instance.reverse();
            println!("[DEBUG] {:?}", instance.first().unwrap())
        }
        //Son 6 ingredientes + 1 header
        assert_eq!(7, result.len())
    }
}
