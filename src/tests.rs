// Copyright 2018 Vlad Yermakov
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use {Interpreter, Node};

use std::collections::BTreeMap;

fn interpret<T: Into<String> + Clone>(text: T) -> String {
    let mut interpreter = Interpreter::with_text(text);
    interpreter.parse().format().1
}

#[test]
fn test_number() {
    fn test_natural() {
        let text = "2";
        assert_eq!("2", interpret(text))
    }

    fn test_rational() {
        let text = "2//3";
        assert_eq!("2 / 3", interpret(text))
    }

    fn test_real() {
        let text = "1.33";
        assert_eq!("1.33", interpret(text))
    }

    fn test_complex() {
        let text = "3i";
        assert_eq!("0 + 3i", interpret(text));
    }

    test_natural();
    test_rational();
    test_real();
    test_complex();
}

#[test]
fn test_add() {
    let text = " 2+2 ";
    assert_eq!("4", interpret(text))
}

#[test]
fn test_add_with_spaces() {
    let text = " 2 + 2 ";
    assert_eq!("4", interpret(text))
}

#[test]
fn test_sub() {
    let text = " 2 - 2 ";
    assert_eq!("0", interpret(text))
}

#[test]
fn test_mul() {
    let text = " 2 * 2 ";
    assert_eq!("4", interpret(text))
}

#[test]
fn test_div() {
    let text = " 2 / 2 ";

    assert_eq!("1", interpret(text))
}

#[test]
fn test_long() {
    let text = " 2 + 2 * 2 - 4 / 2 ";
    assert_eq!("4", interpret(text))
}

#[test]
fn test_ops() {
    fn test_add_complex() {
        let text = "2 + 3i";
        assert_eq!("2 + 3i", interpret(text));
    }

    fn test_add_rational() {
        let text = "2 + 3//4";
        assert_eq!("11 / 4", interpret(text));
    }

    fn test_add_real() {
        let text = "3//4 + 2.5";
        assert_eq!("3.25", interpret(text));
    }

    fn test_add_rationals() {
        let text = "3//8 + 5//8";
        assert_eq!("1 / 1", interpret(text));
    }

    fn test_add_rational_complex() {
        let text = "3//4 + 2i";
        assert_eq!("0.75 + 2i", interpret(text));
    }

    fn test_sub_complex() {
        let text = "3i - 2";
        assert_eq!("-2 + 3i", interpret(text));
    }

    fn test_sub_rational() {
        let text = "2 - 3//4";
        assert_eq!("5 / 4", interpret(text));
    }

    fn test_sub_real() {
        let text = "2.5 - 3//4";
        assert_eq!("1.75", interpret(text));
    }

    fn test_sub_rationals() {
        let text = "3//8 - 5//8";
        assert_eq!("-1 / 4", interpret(text));
    }

    fn test_sub_rational_complex() {
        let text = "3//4 - 2i";
        assert_eq!("0.75 - 2i", interpret(text));
    }

    fn test_mul_complex() {
        let text = "(2 + 3i) * (2 - 3i)";
        assert_eq!("13 + 0i", interpret(text));
    }

    fn test_mul_rational() {
        let text = "2//3 * 3//4";
        assert_eq!("1 / 2", interpret(text));
    }

    fn test_mul_real() {
        let text = "2.5 * 3//4";
        assert_eq!("1.875", interpret(text));
    }

    fn test_mul_rational_complex() {
        let text = "3//4 * (1 + 2i)";
        assert_eq!("0.75 + 1.5i", interpret(text));
    }

    fn test_div_complex() {
        let text = "(1 + 3i) / 2";
        assert_eq!("0.5 + 1.5i", interpret(text));
    }

    fn test_div_rational() {
        let text = "2 / 3//4";
        assert_eq!("8 / 3", interpret(text));
    }

    fn test_div_real() {
        let text = "2.5 / 1//4";
        assert_eq!("10", interpret(text));
    }

    fn test_div_rationals() {
        let text = "3//8 / 5//8";
        assert_eq!("3 / 5", interpret(text));
    }

    fn test_div_rational_complex() {
        let text = "(2 + 1i) / 1//4";
        assert_eq!("8 + 4i", interpret(text));
    }

    test_add_complex();
    test_add_rational();
    test_add_rational_complex();
    test_add_rationals();
    test_add_real();
    test_sub_complex();
    test_sub_rational();
    test_sub_rational_complex();
    test_sub_rationals();
    test_sub_real();
    test_div_complex();
    test_div_rational();
    test_div_rational_complex();
    test_div_rationals();
    test_div_real();
    test_mul_complex();
    test_mul_rational();
    test_mul_rational_complex();
    test_mul_real();
}

#[test]
fn test_bad_numbers() {
    let text = "2.5 - 3.2i";
    assert_eq!("2.5 - 3.2i", interpret(text))
}

#[test]
fn test_parentheses() {
    let text = " (2 + 2) ";
    assert_eq!("4", interpret(text))
}

#[test]
fn test_priority() {
    let text = " 2 + 2 * 2 ";
    assert_eq!("6", interpret(text));

    let text = " 2 + (2 * 2) ";
    assert_eq!("6", interpret(text));

    let text = " (2 + 2) * 2 ";
    assert_eq!("8", interpret(text));
}

#[test]
fn test_unary() {
    let text = " -2 ";
    assert_eq!("-2", interpret(text));

    let text = " +2 ";
    assert_eq!("2", interpret(text));

    let text = " --2 ";
    assert_eq!("2", interpret(text));
}

#[test]
fn test_multiline() {
    let text = "2 + 2";
    assert_eq!("4", interpret(text));
}

#[test]
fn test_function() {
    let text = "fn inc() { 1 }";
    let mut interpreter = Interpreter::with_text(text);
    let func = interpreter.parser.line();
    assert_eq!(1, interpreter.parser.functions.len());
    if let Node::Function { name, .. } = func {
        assert_eq!("inc", name);
    } else {
        panic!("can't parse function");
    }
}

#[test]
fn test_function_with_arguments() {
    let text = "fn inc(num) { num + 1 }";
    let mut interpreter = Interpreter::with_text(text);
    let func = interpreter.parser.line();
    assert_eq!(1, interpreter.parser.functions.len());
    if let Node::Function {
        name,
        arguments,
        body,
    } = func
    {
        assert_eq!("inc", name);
        assert_eq!(vec!["num".to_string()], arguments);
        println!("{:?}", body);
    } else {
        panic!("can't parse function");
    }
}

#[test]
fn test_if() {
    let text = r#"
if 2 < 3 {
    1
} else {
    2
}
"#;
    assert_eq!("1", interpret(text));

    let text = r#"
if 2 > 3 {
    1
} else {
    2
}
"#;
    assert_eq!("2", interpret(text));

    let text = r#"
if 2 > 3 {
    1
} else {
    if 1 > 3 {
        2
    } else {
        3
    }
}
"#;
    assert_eq!("3", interpret(text))
}

#[test]
fn test_conditions() {
    fn test_simple() {
        let text = "2 < 3";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(true));

        let text = "2 > 3";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(false));
    }

    fn test_parentheses() {
        let text = "(2 < 3)";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(true));
    }

    fn test_and() {
        let text = "2 < 3 & 1 < 4";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(true));
    }

    fn test_or() {
        let text = "2 < 3 | 4 > 1";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(true));
    }

    fn test_xor() {
        let text = "2 < 3 ^ 4 < 1";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(true));
    }

    fn test_not() {
        let text = "! 3 < 2";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(true));
    }

    fn test_equals() {
        let text = "1 = 1";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(true));

        let text = "1 = 2";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(false));
    }

    fn test_not_equals() {
        let text = "1 != 2";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(true));

        let text = "1 != 1";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(false));
    }

    fn test_less_than() {
        let text = "1 < 2";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(true));

        let text = "2 < 1";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(false));
    }

    fn test_greater_than() {
        let text = "2 > 1";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(true));

        let text = "1 > 2";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(false));
    }

    fn test_less_than_or_equal() {
        let text = "1 <= 2";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(true));

        let text = "2 <= 1";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(false));
    }

    fn test_greater_than_or_equal() {
        let text = "2 >= 1";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(true));

        let text = "1 >= 2";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(false));
    }

    fn test_bool() {
        let text = "true";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(true));

        let text = "false";
        let mut interpreter = Interpreter::with_text(text);
        assert_eq!(interpreter.parser.condition().is_true(BTreeMap::new()), Some(false))
    }

    test_simple();
    test_parentheses();
    test_and();
    test_or();
    test_xor();
    test_not();
    test_bool();

    test_equals();
    test_not_equals();
    test_less_than();
    test_greater_than();
    test_less_than_or_equal();
    test_greater_than_or_equal();
}
