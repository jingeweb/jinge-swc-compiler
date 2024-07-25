use swc_core::{
  atoms::Atom,
  common::{SyntaxContext, DUMMY_SP},
  ecma::ast::*,
};

use crate::{
  ast::{
    ast_create_arg_expr, ast_create_expr_call, ast_create_expr_ident, ast_create_expr_member,
    ast_create_expr_this,
  },
  common::{
    JINGE_IDENT, JINGE_IMPORT_ROOT_NODES, JINGE_IMPORT_SET_ATTRIBUTE, JINGE_IMPORT_SET_REF,
  },
};

pub fn tpl_set_ref_code(r: Lit) -> Box<Expr> {
  let args = vec![
    ast_create_arg_expr(Box::new(Expr::Lit(r))),
    ast_create_arg_expr(ast_create_expr_ident(JINGE_IDENT)),
    ast_create_arg_expr(ast_create_expr_this()),
  ];
  Box::new(Expr::Call(CallExpr {
    ctxt: SyntaxContext::empty(),
    span: DUMMY_SP,
    callee: Callee::Expr(ast_create_expr_member(
      ast_create_expr_this(),
      MemberProp::Computed(ComputedPropName {
        span: DUMMY_SP,
        expr: ast_create_expr_ident(JINGE_IMPORT_SET_REF.1),
      }),
    )),
    args,
    type_args: None,
  }))
}

pub fn tpl_push_ele_code() -> Box<Expr> {
  let args = vec![ast_create_arg_expr(ast_create_expr_ident(JINGE_IDENT))];
  Box::new(Expr::Call(CallExpr {
    ctxt: SyntaxContext::empty(),
    span: DUMMY_SP,
    callee: Callee::Expr(ast_create_expr_member(
      ast_create_expr_member(
        ast_create_expr_this(),
        MemberProp::Computed(ComputedPropName {
          span: DUMMY_SP,
          expr: ast_create_expr_ident(JINGE_IMPORT_ROOT_NODES.1),
        }),
      ),
      MemberProp::Ident(IdentName::from("push")),
    )),
    args,
    type_args: None,
  }))
}

pub fn tpl_lit_obj(lit_arr: Vec<(IdentName, Box<Expr>)>) -> Box<Expr> {
  Box::new(Expr::Object(ObjectLit {
    span: DUMMY_SP,
    props: lit_arr
      .into_iter()
      .map(|(prop, value)| {
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Str(Str::from(prop.sym)),
          value,
        })))
      })
      .collect(),
  }))
}

pub fn tpl_set_attribute(el: Box<Expr>, attr_name: Atom, attr_value: Box<Expr>) -> Box<Expr> {
  ast_create_expr_call(
    ast_create_expr_ident(JINGE_IMPORT_SET_ATTRIBUTE.1),
    vec![
      ast_create_arg_expr(el),
      ast_create_arg_expr(Box::new(Expr::Lit(Lit::Str(Str::from(attr_name))))),
      ast_create_arg_expr(attr_value),
    ],
  )
}

pub fn tpl_set_idl_attribute(el: Box<Expr>, attr_name: Atom, attr_value: Box<Expr>) -> Box<Expr> {
  Box::new(Expr::Assign(AssignExpr {
    span: DUMMY_SP,
    op: AssignOp::Assign,
    left: AssignTarget::Simple(SimpleAssignTarget::Member(MemberExpr {
      span: DUMMY_SP,
      obj: el,
      prop: MemberProp::Ident(IdentName::from(attr_name)),
    })),
    right: attr_value,
  }))
}
