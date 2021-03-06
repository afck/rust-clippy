use rustc::hir::*;
use rustc::lint::*;
use syntax::ast::LitKind;
use utils::{is_direct_expn_of, match_path, paths, span_lint};

/// **What it does:** This lint checks for missing parameters in `panic!`.
///
/// **Known problems:** Should you want to use curly brackets in `panic!` without any parameter,
/// this lint will warn.
///
/// **Example:**
/// ```
/// panic!("This `panic!` is probably missing a parameter there: {}");
/// ```
declare_lint! {
    pub PANIC_PARAMS, Warn, "missing parameters in `panic!`"
}

#[allow(missing_copy_implementations)]
pub struct PanicPass;

impl LintPass for PanicPass {
    fn get_lints(&self) -> LintArray {
        lint_array!(PANIC_PARAMS)
    }
}

impl LateLintPass for PanicPass {
    fn check_expr(&mut self, cx: &LateContext, expr: &Expr) {
        if_let_chain! {[
            let ExprBlock(ref block) = expr.node,
            let Some(ref ex) = block.expr,
            let ExprCall(ref fun, ref params) = ex.node,
            params.len() == 2,
            let ExprPath(None, ref path) = fun.node,
            match_path(path, &paths::BEGIN_UNWIND),
            let ExprLit(ref lit) = params[0].node,
            is_direct_expn_of(cx, params[0].span, "panic").is_some(),
            let LitKind::Str(ref string, _) = lit.node,
            let Some(par) = string.find('{'),
            string[par..].contains('}')
        ], {
            span_lint(cx, PANIC_PARAMS, params[0].span,
                      "you probably are missing some parameter in your format string");
        }}
    }
}
