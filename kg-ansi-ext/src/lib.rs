#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

use syntax::parse::token;
use syntax::tokenstream::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager, get_exprs_from_tts};
use syntax::ext::build::AstBuilder;  // A trait for expr_usize.
use syntax::ext::quote::rt::{Span, ExtParseUtils, ToTokens};
use syntax::print::pprust::expr_to_string;

use rustc_plugin::Registry;

use std::fmt::Write;

fn expand_rn(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
             -> Box<MacResult + 'static> {
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

    MacEager::expr(cx.parse_expr(s))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("cprintln", expand_rn);
}
