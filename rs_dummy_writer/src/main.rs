use rust_xlsxwriter::{Format, Workbook};
mod users;

fn write_to_excel(num: i32) {
    // generate dummy data
    let users = users::get_users(num);

    // init workbook
    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();

    // write header
    let header_format = Format::new().set_bold();

    sheet
        .write_string_with_format(0, 0, "name", &header_format)
        .unwrap();
    sheet
        .write_string_with_format(0, 1, "age", &header_format)
        .unwrap();
    sheet
        .write_string_with_format(0, 2, "id", &header_format)
        .unwrap();

    for (i, user) in users.iter().enumerate() {
        let (name, age, id) = user;
        sheet.write((i as u32) + 1, 0, name).unwrap();
        sheet.write((i as u32) + 1, 1, *age as f64).unwrap();
        sheet.write((i as u32) + 1, 2, *id as f64).unwrap();
    }

    workbook.save("users.xlsx").unwrap();
}

fn main() {
    write_to_excel(10000);
}
