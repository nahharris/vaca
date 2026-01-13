use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use std::thread;

use tempfile::tempdir;

use super::*;
use crate::bezerro::error::UseError;
use crate::bezerro::value::Value;
use crate::bezerro::{register_builtins, Env};

fn eval_program(src: &str) -> Result<String, crate::bezerro::error::EvalError> {
    // IMPORTANT (Windows): deep recursion can overflow the OS thread stack before our
    // MAX_STACK_DEPTH guard triggers. Run evaluation on a larger stack so we reliably
    // get EvalError::StackOverflow instead of a process-crashing stack overflow.
    let src = src.to_string();
    thread::Builder::new()
        .name("vaca-test-eval".to_string())
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            let nodes = crate::parse(&src).expect("parse should succeed");
            let env = Rc::new(RefCell::new(Env::new()));
            register_builtins(&mut env.borrow_mut());

            let mut last = Value::Nil;
            for node in &nodes {
                last = eval(node, &env)?;
            }
            Ok(last.to_string())
        })
        .expect("spawn eval thread")
        .join()
        .expect("eval thread panicked")
}

fn eval_in_dir(dir: &Path, src: &str) -> Result<String, crate::bezerro::error::EvalError> {
    let src = src.to_string();
    let dir = dir.to_path_buf();
    thread::Builder::new()
        .name("vaca-test-eval-use".to_string())
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            let nodes = crate::parse(&src).expect("parse should succeed");
            let env = Rc::new(RefCell::new(Env::new()));
            register_builtins(&mut env.borrow_mut());
            env.borrow_mut().set_source_dir(dir);

            let mut last = Value::Nil;
            for node in &nodes {
                last = eval(node, &env)?;
            }
            Ok(last.to_string())
        })
        .expect("spawn eval thread")
        .join()
        .expect("eval thread panicked")
}

fn eval_snippet(
    env: &Rc<RefCell<Env>>,
    src: &str,
) -> Result<Value, crate::bezerro::error::EvalError> {
    let nodes = crate::parse(src).expect("parse should succeed");
    let mut last = Value::Nil;
    for node in &nodes {
        last = eval(node, env)?;
    }
    Ok(last)
}

#[test]
fn keyword_equality_preserves_namespace_vs_slash_in_name() {
    let v = eval_program("(== `te/st`: :te/st)").unwrap();
    assert_eq!(v, "false");
}

#[test]
fn let_uses_map_bindings() {
    let v = eval_program("(let {x 10 y 32} (+ x y))").unwrap();
    assert_eq!(v, "42");
}

#[test]
fn recur_allows_deep_tail_recursion_without_stack_overflow() {
    let v = eval_program(
        r#"
        (defn down [n]
          (if (< n 1)
            0
            (recur (- n 1))))
        (down 20000)
        "#,
    )
    .unwrap();

    assert_eq!(v, "0");
}

#[test]
fn loop_allows_deep_iteration_without_stack_overflow() {
    let v = eval_program(
        r#"
        (loop [n 20000]
          (if (< n 1)
            0
            (recur (- n 1))))
        "#,
    )
    .unwrap();

    assert_eq!(v, "0");
}

#[test]
fn non_tail_recursion_is_stopped_before_host_stack_overflow() {
    let err = eval_program(
        r#"
        (defn bad [n]
          (if (< n 1)
            0
            (bad (- n 1))))
        (bad 20000)
        "#,
    )
    .unwrap_err();

    assert!(matches!(
        err,
        crate::bezerro::error::EvalError::StackOverflow { limit: 10_000 }
    ));
}

#[test]
fn use_imports_all_exports() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("mod.vaca"),
        r#"
        (def x 1)
        (defn inc [n] (+ n 1))
        "#,
    )
    .unwrap();

    let v = eval_in_dir(
        dir.path(),
        r#"
        (use mod)
        (inc x)
        "#,
    )
    .unwrap();
    assert_eq!(v, "2");
}

#[test]
fn use_imports_selected_symbols_only() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("mod.vaca"),
        r#"
        (def x 1)
        (defn inc [n] (+ n 1))
        "#,
    )
    .unwrap();

    // importing only inc: x should be undefined
    let err = eval_in_dir(
        dir.path(),
        r#"
        (use mod [inc])
        x
        "#,
    )
    .unwrap_err();
    assert!(matches!(
        err,
        crate::bezerro::error::EvalError::UndefinedSymbol(_)
    ));

    let v = eval_in_dir(
        dir.path(),
        r#"
        (use mod [inc])
        (inc 1)
        "#,
    )
    .unwrap();
    assert_eq!(v, "2");
}

#[test]
fn use_aliasing_works_and_original_name_is_not_required() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("mod.vaca"),
        r#"
        (defn inc [n] (+ n 1))
        "#,
    )
    .unwrap();

    let v = eval_in_dir(
        dir.path(),
        r#"
        (use mod [inc :as plus1])
        (plus1 1)
        "#,
    )
    .unwrap();
    assert_eq!(v, "2");

    let err = eval_in_dir(
        dir.path(),
        r#"
        (use mod [inc :as plus1])
        (inc 1)
        "#,
    )
    .unwrap_err();
    assert!(matches!(
        err,
        crate::bezerro::error::EvalError::UndefinedSymbol(_)
    ));
}

#[test]
fn use_errors_on_missing_export() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("mod.vaca"), "(def x 1)\n").unwrap();

    let err = eval_in_dir(
        dir.path(),
        r#"
        (use mod [missing])
        "#,
    )
    .unwrap_err();
    assert!(matches!(
        err,
        crate::bezerro::error::EvalError::Use(UseError::MissingExport { .. })
    ));
}

#[test]
fn use_macros_work_even_if_helpers_are_not_imported() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("m.vaca"),
        r#"
        (defn helper [n] (+ n 1))
        (defmacro m []
          (quote (helper 10)))
        "#,
    )
    .unwrap();

    let v = eval_in_dir(
        dir.path(),
        r#"
        (use m [m])
        (m)
        "#,
    )
    .unwrap();
    assert_eq!(v, "11");
}

#[test]
fn use_does_not_rewrite_quote_in_non_macro_context() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("q.vaca"),
        r#"
        (def x 1)
        (def sym (quote x))
        "#,
    )
    .unwrap();

    // `sym` should literally be the symbol `x`, not the mangled name.
    let v = eval_in_dir(
        dir.path(),
        r#"
        (use q [sym])
        sym
        "#,
    )
    .unwrap();
    assert_eq!(v, "x");
}

#[test]
fn use_rewrite_is_binder_aware_in_macro_expansions() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("b.vaca"),
        r#"
        (def x 100)
        (defn id [n] n)
        (defmacro m []
          (quote (let {x 1} (id x))))
        "#,
    )
    .unwrap();

    let v = eval_in_dir(
        dir.path(),
        r#"
        (use b [m])
        (m)
        "#,
    )
    .unwrap();
    assert_eq!(v, "1");
}

#[test]
fn use_super_resolves_parent_directory() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("sub")).unwrap();
    fs::write(dir.path().join("mod.vaca"), "(def x 9)\n").unwrap();

    let v = eval_in_dir(
        &dir.path().join("sub"),
        r#"
        (use super.mod [x])
        x
        "#,
    )
    .unwrap();
    assert_eq!(v, "9");
}

#[test]
fn use_detects_cycles() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a.vaca"), "(use b)\n(def x 1)\n").unwrap();
    fs::write(dir.path().join("b.vaca"), "(use a)\n(def y 2)\n").unwrap();

    let err = eval_in_dir(
        dir.path(),
        r#"
        (use a)
        "#,
    )
    .unwrap_err();
    assert!(matches!(
        err,
        crate::bezerro::error::EvalError::Use(UseError::CyclicUse { .. })
    ));
}

#[test]
fn use_errors_on_name_collisions() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("m.vaca"), "(def x 1)\n").unwrap();

    let err = eval_in_dir(
        dir.path(),
        r#"
        (def x 0)
        (use m [x])
        "#,
    )
    .unwrap_err();
    assert!(matches!(
        err,
        crate::bezerro::error::EvalError::Use(UseError::NameCollision { .. })
    ));
}

#[test]
fn use_caches_module_evaluation() {
    let dir = tempdir().unwrap();
    let module_path = dir.path().join("mod.vaca");
    fs::write(&module_path, "(def x 1)\n").unwrap();

    let dir_path = dir.path().to_path_buf();
    thread::Builder::new()
        .name("vaca-test-use-cache".to_string())
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            let env = Rc::new(RefCell::new(Env::new()));
            register_builtins(&mut env.borrow_mut());
            env.borrow_mut().set_source_dir(dir_path.clone());

            let v1 = eval_snippet(
                &env,
                r#"
                (use mod [x :as x1])
                x1
                "#,
            )?;
            assert_eq!(v1.to_string(), "1");

            // Change the file: if `use` re-evaluated, x2 would become 2. With cache, it stays 1.
            fs::write(dir_path.join("mod.vaca"), "(def x 2)\n").unwrap();

            let v2 = eval_snippet(
                &env,
                r#"
                (use mod [x :as x2])
                x2
                "#,
            )?;
            assert_eq!(v2.to_string(), "1");

            Ok::<(), crate::bezerro::error::EvalError>(())
        })
        .unwrap()
        .join()
        .unwrap()
        .unwrap();
}
