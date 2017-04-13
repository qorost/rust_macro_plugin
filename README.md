#Rust Macro and Compiler Plugin Conflicts

This project is to illustrate the macro and compiler plugin conflicts.

## Projects Directory


The project is arranged as below:

* `src`, source code that generate error
* `myplugin`, the plugin code
* `myplugin/support`, defines the macros used in the source code.


In the `src/main.rs`, it used the `macros` defined in `myplugin/support`. The `extend!` macro is used to add a field in the struct definition. The `mark!` macro is used to fill the added field in declaration.

## Compilation
The code compiles well, if the `#[check]` attribute is applied to the single function item. But if we use `#![check]` it failed with:
```
error[E0308]: mismatched types
  --> src/main.rs:22:17
     |
     22 |     let x:AA =  mark!(AA{x:32,});
        |                 ^^^^^^^^^^^^^^^^ expected struct `test_macro::AA`, found bool
           |
              = note: expected type `test_macro::AA`
                            found type `bool`

                            error: aborting due to previous error

                            error: Could not compile `fine-unsafe`.
```

The reason of the code is originated from the code in `myplugin/src/lib.rs`, it use `fold` (`src/libsyntax/fold.rs`) to iterate over the items of the ast tree. It fails to extend the macros successfully.
```
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
```
