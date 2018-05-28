//use syntax::ast::*;
use rustc::lint::*;
use rustc::hir::*;
use utils::{snippet, span_lint, span_lint_and_sugg};
use utils::sugg::Sugg;

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
}

impl<'a, 'tcx>  LateLintPass<'a, 'tcx>  for ItemLength {
	fn check_expr(&mut self, cx: &LateContext<'a, 'tcx>, e: &'tcx Expr) {
		///
		span_lint_and_sugg(
			cx,
			ITEM_LENGTH,
			e.span,
			"this just fails",
			"suggestion here",
			"".to_string()
		);
	}
}