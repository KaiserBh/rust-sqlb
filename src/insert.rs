use crate::{into_returnings, sql_comma_names, sql_comma_params, sql_returnings, Field, SqlBuilder, Val};

pub fn insert(table: &str) -> SqlInsertBuilder {
	SqlInsertBuilder {
		table: table.to_string(),
		data: None,
		returnings: None,
	}
}

#[derive(Clone)]
pub struct SqlInsertBuilder {
	table: String,
	data: Option<Vec<Field>>,
	returnings: Option<Vec<String>>,
}

impl SqlInsertBuilder {
	pub fn data(mut self, fields: Vec<Field>) -> Self {
		self.data = Some(fields);
		self
	}

	pub fn returning(mut self, names: &[&str]) -> Self {
		self.returnings = into_returnings(self.returnings, names);
		self
	}
}

impl SqlBuilder for SqlInsertBuilder {
	fn sql(&self) -> String {
		// SQL: INSERT INTO table_name (name1, ...) VALUES ($1, ...) RETURNING r1, ...;

		// SQL: INSERT INTO table_name
		let mut sql = String::from(format!("INSERT INTO \"{}\" ", self.table));

		// Note: empty data is a valid usecase, if the row has a all required field with default or auto gen.
		if let Some(fields) = &self.data {
			// SQL: (name1, name2, ...)
			sql.push_str(&format!("({}) ", sql_comma_names(fields)));

			// SQL: VALUES ($1, $2, ...)
			sql.push_str(&format!("VALUES ({}) ", sql_comma_params(fields)));
		}

		// SQL: RETURNING "r1", "r2", ...
		if let Some(returnings) = &self.returnings {
			sql.push_str(&format!("RETURNING {} ", sql_returnings(returnings)));
		}

		sql
	}

	fn vals(&self) -> Vec<Val> {
		match &self.data {
			Some(fields) => fields.iter().map(|f| f.1.clone()).collect(),
			None => Vec::new(),
		}
	}
}
