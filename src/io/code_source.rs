use std::path::{PathBuf, Path};
use crate::funge_dialect::FungeDialect;

pub struct CodeSource {
	path: PathBuf,
	dialect: FungeDialect,
}
impl CodeSource {
	pub fn get_path(&self) -> &Path {
		self.path.as_path()
	}
}
