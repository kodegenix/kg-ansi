#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate syntax_pos;
extern crate rustc;
extern crate rustc_plugin;


use rustc_plugin::Registry;
use syntax::ext::base::{ExtCtxt, MacEager, MacResult, get_exprs_from_tts};
use syntax::ext::build::AstBuilder;
use syntax::print::pprust::expr_to_string;
use syntax::tokenstream::TokenTree;
use syntax_pos::Span;

use std::fmt::Write;

fn expand_rn(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
             -> Box<dyn MacResult + 'static> {
    let mut s = String::new();
    s.push_str("println!(");
    if let Some(exprs) = get_exprs_from_tts(cx, sp, args) {
        let mut ite = exprs.iter().peekable();
        while let Some(e) = ite.next() {
            if ite.peek().is_some() {
                write!(s, "{},", expr_to_string(&e));
            } else {
                write!(s, "{});", expr_to_string(&e));
            }
        }
    } else {
        unimplemented!();
    }

    let mut p = cx.new_parser_from_tts(args);
    MacEager::expr(p.parse_expr(s))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("cprintln", expand_rn);
}
