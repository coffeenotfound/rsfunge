use std::path::{PathBuf, Path};
use crate::FungeDialect;

#[derive(Clone)]
pub struct CodeSource {
	path: PathBuf,
	dialect: Option<FungeDialect>,
}

impl CodeSource {
	pub fn new(path: PathBuf, dialect: Option<FungeDialect>) -> Self {
		CodeSource {
			path,
			dialect,
		}
	}
	
	pub fn get_path(&self) -> &Path {
		self.path.as_path()
	}
	
	pub fn get_dialect(&self) -> Option<FungeDialect> {
		self.dialect
	}
}
