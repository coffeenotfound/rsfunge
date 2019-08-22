use std::path::{PathBuf, Path};
use crate::FungeDialect;

#[derive(Clone)]
pub struct CodeSource {
	path: PathBuf,
	dialect: FungeDialect,
}
impl CodeSource {
	pub fn get_path(&self) -> &Path {
		self.path.as_path()
	}
}
