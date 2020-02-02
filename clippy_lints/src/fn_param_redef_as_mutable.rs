use rustc_lint::{EarlyLintPass, EarlyContext};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use syntax::ast::*;
use syntax::visit::FnKind;
use rustc_span::Span;
use rustc_errors::DiagnosticBuilder;
use crate::utils::{span_lint_and_then, multispan_sugg};
use if_chain::if_chain;

declare_clippy_lint! {
    /// **What it does:**
    ///
    /// **Why is this bad?**
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// // example code
    /// ```
    pub FN_PARAM_REDEF_AS_MUTABLE,
    complexity,
    "default lint description"
}

declare_lint_pass!(FnParamRedefAsMutable => [FN_PARAM_REDEF_AS_MUTABLE]);

impl EarlyLintPass for FnParamRedefAsMutable {
    fn check_fn(&mut self, cx: &EarlyContext<'_>, fn_kind: FnKind<'_>, fn_decl: &FnDecl, span: Span, _: NodeId) {
        if let FnKind::ItemFn(_, _, _, block) | FnKind::Method(_, _, _, block) = fn_kind {
            for stmt in &block.stmts {
                check_statement(cx, fn_decl, span, stmt);
            }
        }
    }
}

fn check_statement(cx: &EarlyContext<'_>, fn_decl: &FnDecl, fn_span: Span, stmt: &Stmt) {
    if_chain! {
        // Check to see if the local variable is defined as mutable
        if let StmtKind::Local(ref local) = stmt.kind;
        if let PatKind::Ident(mode, ..) = local.pat.kind;
        if let BindingMode::ByValue(mutability) = mode;
        if let Mutability::Mut = mutability;

        if let Some(ref expr) = local.init;
        if let ExprKind::Path(_, ref path) = expr.kind;
        if let Some(ref segment) = path.segments.last();
        if let name = segment.ident.name;

        // The path to fn parameters is 1 in length.
        if path.segments.len() == 1;
        then {
            for param in &fn_decl.inputs {
                if_chain! {
                    if let PatKind::Ident(_, ident, ..) = param.pat.kind; 
                    if ident.name == name;

                    then {
                        let sugg = |db: &mut DiagnosticBuilder<'_>| {
                            db.span_help(param.span, "consider making this param `mut`");
                            db.span_help(stmt.span, "consider removing this local variable");

                            multispan_sugg(db, "...".to_string(), vec![]);
                        };

                        span_lint_and_then(
                            cx,
                            FN_PARAM_REDEF_AS_MUTABLE,
                            fn_span,
                            "a parameter was redefined as mutable, can be removed",
                            sugg,
                        );
                    }
                }
            }
        }
    }
}
