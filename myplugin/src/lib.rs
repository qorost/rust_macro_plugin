#![crate_type = "dylib"]
#![feature(plugin_registrar, quote, rustc_private)]
#![allow(dead_code)]
#![allow(unused_imports)]



extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;
extern crate syntax_pos;

//use syntax::ast::{self, StructField, Unsafety, Ident};
use syntax::ast::*;
use syntax::ast;
use syntax_pos::Span;
use syntax::codemap::{BytePos, Spanned};

use syntax::ext::base::*;
use syntax::ext::quote::rt::ToTokens;
use syntax::parse::{self, token};
use syntax::ptr::P;
use syntax::symbol::Symbol;
use syntax::tokenstream::TokenTree;
use rustc_plugin::Registry;


//for fold
use std::fmt::{self, Display, Formatter};
use syntax::fold::{self, Folder};
use syntax::ext::build::AstBuilder;
/*use syntax::ast::{BinOpKind, Block, Expr, ExprKind, Item, ItemKind, Lit, Mod, 
                  LitKind, Mac, MetaItem, MetaItemKind, NestedMetaItemKind,
                  Path, PathSegment, Stmt, StmtKind, UnOp,FnDecl};
*/
//use syntax::ast::*;
use syntax::util::small_vector::SmallVector;
use syntax::print::pprust;
use std::collections::HashSet;



struct Refchecker<'a, 'cx: 'a> {
    cx: &'a mut ExtCtxt<'cx>,
}


impl<'a,'cx> Folder for Refchecker<'a, 'cx> {
    fn fold_expr(&mut self, e: P<Expr>) -> P<Expr> {
        //ok with the original
        //e.map(|e| fold::noop_fold_expr(e, self))
        
        match e.unwrap() {
            //This expression cause error
            e@Expr {node: ExprKind::Mac(_), ..} => {
                let expanded = self.cx.expander().fold_expr(P(e));
                self.fold_expr(expanded)
            }
            e => P(fold::noop_fold_expr(e,self)), 
        }
    }

    fn fold_stmt(&mut self, s: Stmt) -> SmallVector<Stmt> {
        fold::noop_fold_stmt(s,self)
    }

    ///modify this to instrument code
    /// FIXME
    fn fold_item(&mut self, i: P<Item>) -> SmallVector<P<Item>> {
        fold::noop_fold_item(i,self)
    }

    fn fold_item_simple(&mut self, i: Item) -> Item {
        fold::noop_fold_item_simple(i,self)
    }

    fn fold_item_kind(&mut self, i: ItemKind) -> ItemKind {
        fold::noop_fold_item_kind(i,self)
    }

    /// 
    fn fold_block(&mut self, b: P<Block>) -> P<Block> {
        fold::noop_fold_block(b,self)
    }


    fn fold_fn_decl(&mut self, d: P<FnDecl>) -> P<FnDecl> {
        fold::noop_fold_fn_decl(d,self)
    }

    fn fold_mac(&mut self, mac: Mac) -> Mac {
        mac
    }

    fn fold_mod(&mut self, m: Mod) -> Mod {
        fold::noop_fold_mod(m, self)
    }

    fn fold_foreign_item(&mut self, ni: ForeignItem) -> ForeignItem {
        fold::noop_fold_foreign_item(ni, self)
    }
}




#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    #[cfg(feature = "debugmyplugin")]
    println!("DEBUG: IN the plugin_registrar expand_checkderef");
    reg.register_syntax_extension(
        Symbol::intern("check"),
        MultiModifier(Box::new(|cx: &mut ExtCtxt, _span: Span, _: &MetaItem, a: Annotatable| {
            
            let mut checker = &mut Refchecker {
                cx: cx,
            };
            match a {
                Annotatable::Item(i) => Annotatable::Item (
                    checker.fold_item(i).expect_one("expected exactly one item")),
                _ => a,
            }
        })));
}
