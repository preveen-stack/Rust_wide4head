extern crate mysql;
use mysql as my;

#[derive(Debug, PartialEq, Eq)]
struct RawData {
	unit_identifier : String,
	raw_data : String,	
}
fn main() {
	let pool = my::Pool::new("mysql://seroot:admin@www.sesolarpumps.com:3306").unwrap();
	let selected_raw_data : Vec<RawData> = pool.prep_exec("select unit_identifier, raw_data from se_data_monitor.unit_in_data  order by id desc limit 12", ()).map(|result| {
	result.map(|x| x.unwrap()).map(|row| {
		let (unit_identifier, raw_data) = my::from_row(row);
	RawData {
		unit_identifier : unit_identifier,
		raw_data : raw_data,	
	}
	}).collect()
}).unwrap();
println!(selected_raw_data);
println!("Yay");
}
