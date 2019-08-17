use std::collections::HashMap;

// Real enum pattern (total bruh moment)
pub struct FungeDialect {
	pub dialect_id: &'static str,
}
impl FungeDialect {
	pub const BEFUNGE_93: FungeDialect = FungeDialect { dialect_id: "befunge93" };
	pub const BEFUNGE_98: FungeDialect = FungeDialect { dialect_id: "befunge98" };
	pub const UNEFUNGE_98: FungeDialect = FungeDialect { dialect_id: "unefunge98" };
	pub const TREFUNGE_98: FungeDialect = FungeDialect { dialect_id: "trefunge98" };
}

lazy_static!(
	static ref FUNGE_DIALECT_MAP: HashMap<&'static str, &'static FungeDialect> = {
		let mut map: HashMap<&str, &FungeDialect> = HashMap::new();
		map.insert(FungeDialect::BEFUNGE_93.dialect_id, &FungeDialect::BEFUNGE_93);
		map
	};
);
