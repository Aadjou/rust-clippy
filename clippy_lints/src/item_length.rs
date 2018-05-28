use syntax::ast::*;
use rustc::lint::{LateContext, LateLintPas, LintArray, LintPass};

declare_clippy_lint! {
	pub ITEM_LENGTH, 
	complexity,
	"Warn on .... please fix"
}

#[derive(Copy,Clone)]
pub struct ItemLength;

impl LintPass for ItemLength {
	fn get_lints(&self) -> LintArray {
		lint_array!(ITEM_LENGTH)
	}

	fn check_expr(&mut self, cx: &LateContext, exp: &Expr) {
		// insert check here
	}
}