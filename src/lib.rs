use std::collections::HashSet;

trait Relation<T, W> {
	fn operation(&self, a: T, b: T) -> bool;

	fn is_transitive(&self) -> bool;

	fn is_reflexive(&self) -> bool;

	fn is_symetric(&self) -> bool;

	fn is_equivalence(&self) -> bool {
		self.is_symetric() && self.is_reflexive() && self.is_transitive()
	}

	/// Test if reasoning is correct given a set operations ex [(1, 2), (3, 4)] where the numbers correspond to unknown variables.
	fn valid(&self, reasoning: Vec<(uint, uint)>, nvariables: uint) -> bool {
		if self.is_equivalence() {
			true
		} else {
			let mut facts: HashSet<(uint, uint)> = HashSet::new();
			let mut constrained: Vec<bool> = Vec::new();
			constrained.grow(nvariables, false);
			for &reason in reasoning.iter() {
				if !facts.contains(&reason) {
					if !constrained[reason.0] || !constrained[reason.1] {
						// reason contains an unconstrained variable or the reason is reflexive and have no constraints, add this as a fact
						constrained[reason.0] = true;
						constrained[reason.1] = true;
					} else {
						// Both variables in relation is constrained, should check if constraint is filled before adding as fact
						let mut found = false;
						if self.is_reflexive() {
							found = true;
						}
						if self.is_symetric() && !found {
							for &fact in facts.iter() {
								if fact.0 == fact.1 && fact.1 == fact.0 {
									found = true;
								}
							}
						}
						if self.is_transitive() && !found {
							let (from, to) = reason;
							if find_path(from, to, &facts) >= 2 {
								found = true;
							}
						}
						if !found {
							return false;
						}
					}
					facts.insert(reason);
					if self.is_symetric() {
						facts.insert((reason.1, reason.0));
					}
				}
			}
			return true;
		}
	}
}

fn find_path(from: uint, to: uint, graph: &HashSet<(uint, uint)>) -> int {
	let mut stack = Vec::new();
	let mut visisted = HashSet::new();
	let mut depth = 0;
	stack.push(from);
	while !stack.is_empty() {
		let visit = stack.pop().unwrap();
		if visit == to {
			return depth;
		}
		depth += 1;
		if !visisted.contains(&visit) {
			visisted.insert(visit);
			for &(f, t) in graph.iter() {
				if f == visit {
					stack.push(t);
				}
			}
		}
	}
	return -1
}


#[test]
fn find_path_test() {
	let mut a = HashSet::new();
	a.insert((1, 2));
	a.insert((2, 3));
	a.insert((3, 4));
	assert!(find_path(1, 4, &a) == 3);
	assert!(find_path(4, 1, &a) == -1);
}


struct And;

impl Relation<bool, bool> for And {
	fn operation(&self, a: bool, b: bool) -> bool {
		a && b
	}

	fn is_transitive(&self) -> bool {
		true
	}
	
	fn is_reflexive(&self) -> bool {
		true
	}
	
	fn is_symetric(&self) -> bool {
		true
	}
}

struct Greater;

impl Relation<int, int> for Greater {
	fn operation(&self, a: int, b: int) -> bool {
		a > b
	}

	fn is_reflexive(&self) -> bool {
		false
	}
	fn is_symetric(&self) -> bool {
		false
	}
	fn is_transitive(&self) -> bool {
		true
	}
}

#[test]
fn is_equivalence() {
	assert!(And.is_equivalence());
	assert!(Greater.is_equivalence() == false);
}

#[test]
fn valid() {
	// Test if this holds, a && b, b && a, b && c, c && b
	let valid_reasoning = vec!((0, 1), (1, 0), (1, 2), (2, 1));
	assert!(And.valid(valid_reasoning, 3));
	// a > b, b > c, a > c
	let valid_reasoning_2 = vec!((0, 1), (1, 2), (0, 2));
	assert!(Greater.valid(valid_reasoning_2, 3));
}

#[test]
fn invalid() {
	// test if this holds, (it should not), a > b, b > a
	let invalid_reasoning = vec!((0, 1), (1, 0));
	assert!(Greater.valid(invalid_reasoning, 2) == false);
}
