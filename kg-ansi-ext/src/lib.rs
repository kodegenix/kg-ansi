#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate syntax_pos;
extern crate rustc;
extern crate rustc_plugin;

use rustc_plugin::Registry;
use syntax::ext::base::*;
use syntax::tokenstream::*;
use syntax::parse::*;
use syntax_pos::*;
use syntax::ast::*;
use syntax::print::pprust::expr_to_string;

use std::fmt::Write;

fn parse_code_expr(cx: &mut ExtCtxt, macro_name: &str, code: String)
            -> Box<dyn MacResult + 'static> {
    let expr = new_parser_from_source_str(
        cx.parse_sess(),
        FileName::Macros(macro_name.to_string()),
        code
    ).parse_expr().unwrap();
    MacEager::expr(expr)
}

fn expand_print(print_name: &str, macro_name: &str, cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
    -> Box<dyn MacResult + 'static> {

    if !args.is_empty() {
        let exprs = get_exprs_from_tts(cx, sp, args).expect("cannot get argument list");
        if let Some((pstr, _)) = syntax::ext::base::expr_to_string(
                    cx,
                    exprs.get(0).unwrap().clone(),
                    "format argument must be a string literal") {
            let mut code = String::new();
            code.push_str(print_name);
            code.push_str("!(\"");
            write!(code, "{}", pstr.as_str().get().escape_default()).unwrap();
            code.push_str("\"");
            for e in exprs.into_iter().skip(1) {
                match e.node {
                    ExprKind::Assign(ref id, ref value) => {
                        eprintln!("assign id {:?} value {:?}", id, value);
                    }
                    _ => {
                        eprintln!("other {:#?}", e);
                    }
                }
                code.push(',');
                write!(code, "{}", expr_to_string(&e)).unwrap();
            }
            code.push(')');
            return parse_code_expr(cx, macro_name, code);
        }
    } else {
        return parse_code_expr(cx, macro_name, format!("{}!()", print_name));
    }

    DummyResult::any(sp)
}

fn expand_cprintln(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
             -> Box<dyn MacResult + 'static> {
    expand_print("println", "kg_ansi_ext::cprintln", cx, sp, args)
}

fn expand_cprint(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
                   -> Box<dyn MacResult + 'static> {
    expand_print("print", "kg_ansi_ext::cprint", cx, sp, args)
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("cprintln", expand_cprintln);
    reg.register_macro("cprint", expand_cprint);
}
