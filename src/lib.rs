trait Relation<T> {
	fn operation(&self, a: T, b: T) -> T;
	fn is_transitive(&self) -> bool;
	fn is_reflective(&self) -> bool;
	fn is_symetric(&self) -> bool;
	fn is_equivalence(&self) -> bool {
		self.is_symetric() && self.is_reflective() && self.is_transitive()
	}
}


struct Addition;

impl Relation<int> for Addition {
	fn operation(&self, a: int, b: int) -> int {
		a + b
	}
	fn is_transitive(&self) -> bool {
		true
	}
	fn is_reflective(&self) -> bool {
		true
	}
	fn is_symetric(&self) -> bool {
		true
	}
}


#[test]
fn operation() {
	let add = Addition;
	assert!(add.operation(5, 6) == 11);
}

#[test]
fn add_is_equivalence() {
	let add = Addition;
	assert!(add.is_equivalence());
}
