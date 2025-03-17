// use duckdb::{Connection, Result as DuckResult};
// use crate::types::{InMemoryRegistry,TypeRegistery,ClassID,ClassMeta,Property};

// #[test]
// fn db_connects() {
// 	Connection::open_in_memory().unwrap();
// 	Connection::open_in_memory().unwrap();
// }

// pub struct SimState<'code>{
// 	pub reg:InMemoryRegistry<'code>,
// 	pub con:Connection
// }

// impl<'code> SimState<'code>{
// 	fn new_raw(reg:InMemoryRegistry<'code>) -> DuckResult<Self>{
// 		let con =  Connection::open_in_memory()?;
// 		Ok(Self{con,reg})
// 	}

// 	pub fn new(reg:InMemoryRegistry<'code>) -> DuckResult<Self>{
// 		let ans = Self::new_raw(reg)?;
// 		for i in 0..ans.reg.get_cur_class_id(){
// 			let class = ans.reg.get_class(i).unwrap();
// 		}

// 		todo!()
// 	}
// }

