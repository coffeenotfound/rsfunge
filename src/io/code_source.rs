use std::path::PathBuf;
use crate::funge_dialect::FungeDialect;

pub struct CodeSource {
	path: PathBuf,
	dialect: FungeDialect,
}
