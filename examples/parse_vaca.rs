//! Parses a small Vaca program and prints the EDN AST.
//!
//! Run with:
//! ```bash
//! cargo run --example parse_vaca
//! ```

fn main() {
    // This sample is valid EDN + Vaca's typing-through-tags convention.
    let input = r#"
(use stl.io [println])

(defn #int sum [#int a #int b]
  :doc "Takes 2 numbers and add them together"
  (+ a b))

(defn #void main []
  (println "Hello World"))
"#;

    match vaca::parse(input) {
        Ok(nodes) => println!("{:#?}", nodes),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}
