// run-rustfix

#![deny(clippy::internal)]
#![feature(rustc_private)]

extern crate rustc;
extern crate rustc_hir;
extern crate rustc_lint;
#[macro_use]
extern crate rustc_session;
use rustc_hir::Expr;
use rustc_lint::{LateContext, LateLintPass};

declare_lint! {
    pub TEST_LINT,
    Warn,
    ""
}

declare_lint_pass!(Pass => [TEST_LINT]);

impl<'a, 'tcx> LateLintPass<'a, 'tcx> for Pass {
    fn check_expr(&mut self, _cx: &LateContext<'a, 'tcx>, expr: &'tcx Expr) {
        let _ = expr.span.ctxt().outer_expn().expn_data();
    }
}

fn main() {}
