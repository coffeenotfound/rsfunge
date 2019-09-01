use std::path::{PathBuf, Path};
use crate::FungeDialect;

#[derive(Clone)]
pub struct CodeSource {
	path: PathBuf,
	dialect: FungeDialect,
}
impl CodeSource {
	pub fn new(path: PathBuf, dialect: FungeDialect) -> Self {
		CodeSource {
			path,
			dialect,
		}
	}
	
	pub fn get_path(&self) -> &Path {
		self.path.as_path()
	}
	
	pub fn get_dialect(&self) -> FungeDialect {
		self.dialect
	}
}
