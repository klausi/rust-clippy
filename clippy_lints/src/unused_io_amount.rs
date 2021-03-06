// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::rustc::hir;
use crate::rustc::lint::{LateContext, LateLintPass, LintArray, LintPass};
use crate::rustc::{declare_tool_lint, lint_array};
use crate::utils::{is_try, match_qpath, match_trait_method, paths, span_lint};

/// **What it does:** Checks for unused written/read amount.
///
/// **Why is this bad?** `io::Write::write` and `io::Read::read` are not
/// guaranteed to
/// process the entire buffer. They return how many bytes were processed, which
/// might be smaller
/// than a given buffer's length. If you don't need to deal with
/// partial-write/read, use
/// `write_all`/`read_exact` instead.
///
/// **Known problems:** Detects only common patterns.
///
/// **Example:**
/// ```rust,ignore
/// use std::io;
/// fn foo<W: io::Write>(w: &mut W) -> io::Result<()> {
///     // must be `w.write_all(b"foo")?;`
///     w.write(b"foo")?;
///     Ok(())
/// }
/// ```
declare_clippy_lint! {
    pub UNUSED_IO_AMOUNT,
    correctness,
    "unused written/read amount"
}

pub struct UnusedIoAmount;

impl LintPass for UnusedIoAmount {
    fn get_lints(&self) -> LintArray {
        lint_array!(UNUSED_IO_AMOUNT)
    }
}

impl<'a, 'tcx> LateLintPass<'a, 'tcx> for UnusedIoAmount {
    fn check_stmt(&mut self, cx: &LateContext<'_, '_>, s: &hir::Stmt) {
        let expr = match s.node {
            hir::StmtKind::Semi(ref expr, _) | hir::StmtKind::Expr(ref expr, _) => &**expr,
            _ => return,
        };

        match expr.node {
            hir::ExprKind::Match(ref res, _, _) if is_try(expr).is_some() => {
                if let hir::ExprKind::Call(ref func, ref args) = res.node {
                    if let hir::ExprKind::Path(ref path) = func.node {
                        if match_qpath(path, &paths::TRY_INTO_RESULT) && args.len() == 1 {
                            check_method_call(cx, &args[0], expr);
                        }
                    }
                } else {
                    check_method_call(cx, res, expr);
                }
            },

            hir::ExprKind::MethodCall(ref path, _, ref args) => match &*path.ident.as_str() {
                "expect" | "unwrap" | "unwrap_or" | "unwrap_or_else" => {
                    check_method_call(cx, &args[0], expr);
                },
                _ => (),
            },

            _ => (),
        }
    }
}

fn check_method_call(cx: &LateContext<'_, '_>, call: &hir::Expr, expr: &hir::Expr) {
    if let hir::ExprKind::MethodCall(ref path, _, _) = call.node {
        let symbol = &*path.ident.as_str();
        if match_trait_method(cx, call, &paths::IO_READ) && symbol == "read" {
            span_lint(
                cx,
                UNUSED_IO_AMOUNT,
                expr.span,
                "handle read amount returned or use `Read::read_exact` instead",
            );
        } else if match_trait_method(cx, call, &paths::IO_WRITE) && symbol == "write" {
            span_lint(
                cx,
                UNUSED_IO_AMOUNT,
                expr.span,
                "handle written amount returned or use `Write::write_all` instead",
            );
        }
    }
}
