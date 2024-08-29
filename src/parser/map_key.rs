use swc_common::{Spanned, DUMMY_SP};
use swc_core::{atoms::Atom, ecma::ast::*};

use crate::ast::ast_create_expr_ident;

use super::{emit_error, JINGE_LOOP_EACH_DATA, JINGE_LOOP_EACH_INDEX, JINGE_LOOP_KEY};

const BAD_KEY_WARNING: &'static str = "警告：key 的表达式必须是 map 函数的参数或参数的属性表达式";

/// 将 map 函数体返回的第一个有 key 属性的 jsx 元素的 key 属性的表达式，转换成 <For> 组件的 keyFn 属性。
/// 要求 key 属性必须是 Ident 或 MemberExpr，且必须是 map 函数的第一个 data 参数或第二个 index 参数。
///
pub struct KeyFnFindVisitor {
  pub arg_data: Option<Atom>,
  pub arg_index: Option<Atom>,
  pub slot_vm_name: Atom,
}
impl KeyFnFindVisitor {

  fn get_key_from_jsx_element(&self, expr: &JSXElement) -> Option<Box<Expr>> {
    expr.opening.attrs.iter().find_map(|attr| {
      let JSXAttrOrSpread::JSXAttr(attr) = attr else {
        return None;
      };
      if !matches!(&attr.name, JSXAttrName::Ident(id) if JINGE_LOOP_KEY.eq(&id.sym)) {
        return None;
      }
      println!("{:?}", attr.value);
      let Some(JSXAttrValue::JSXExprContainer(expr)) = &attr.value else {
        emit_error(attr.span(), BAD_KEY_WARNING);
        return None;
      };
      let JSXExpr::Expr(expr) = &expr.expr else {
        emit_error(expr.span(), BAD_KEY_WARNING);
        return None;
      };
      match expr.as_ref() {

        Expr::Member(e) => {
          //
        }
        _ => {
          emit_error(expr.span(), BAD_KEY_WARNING);
          None
        }
      }
    })
  }

  fn get_key_from_jsx_fragment(&self, expr: &JSXFragment) -> Option<Box<Expr>> {
    for child in expr.children.iter() {
      let rtn = match child {
        JSXElementChild::JSXExprContainer(e) => match &e.expr {
          JSXExpr::Expr(e) => self.get_key_fn_inner(e.as_ref()),
          JSXExpr::JSXEmptyExpr(_) => None,
        },
        JSXElementChild::JSXElement(e) => self.get_key_from_jsx_element(e.as_ref()),
        JSXElementChild::JSXFragment(fe) => self.get_key_from_jsx_fragment(fe),
        _ => None,
      };
      if rtn.is_some() {
        return rtn;
      }
    }
    None
  }

  fn get_key_fn_inner(&self, expr: &Expr) -> Option<Box<Expr>> {
    match expr {
      Expr::Paren(e) => self.get_key_fn_inner(e.expr.as_ref()),
      Expr::JSXFragment(fe) => self.get_key_from_jsx_fragment(fe),
      Expr::JSXElement(e) => self.get_key_from_jsx_element(e.as_ref()),
      _ => None,
    }
  }

  pub fn get_key_fn(&self, expr: &ArrowExpr) -> Option<Box<Expr>> {
    let BlockStmtOrExpr::Expr(expr) = expr.body.as_ref() else {
      // self.parse_component_element 里会约束 Slot 函数只能是箭头函数且箭头函数直接返回 Expr 表达式。
      // 所以如果是 expr.body 是 BlockStmt 则不再需要尝试获取 key 属性。
      return None;
    };
    self.get_key_fn_inner(expr)
  }
}
