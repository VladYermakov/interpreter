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

#[test]
fn test_number() {
    fn test_natural() {
        let text = "2";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("2", interpreter.interpret())
    }

    fn test_rational() {
        let text = "2//3";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("2 / 3", interpreter.interpret())
    }

    fn test_real() {
        let text = "1.33";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("1.33", interpreter.interpret())
    }

    fn test_complex() {
        let text = "3i";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("0 + 3i", interpreter.interpret());
    }

    test_natural();
    test_rational();
    test_real();
    test_complex();
}

#[test]
fn test_add() {
    let text = " 2+2 ";
    let mut interpreter = Interpreter::new_with_text(text);
    assert_eq!("4", interpreter.interpret())
}

#[test]
fn test_add_with_spaces() {
    let text = " 2 + 2 ";
    let mut interpreter = Interpreter::new_with_text(text);
    assert_eq!("4", interpreter.interpret())
}

#[test]
fn test_sub() {
    let text = " 2 - 2 ";
    let mut interpreter = Interpreter::new_with_text(text);
    assert_eq!("0", interpreter.interpret())
}

#[test]
fn test_mul() {
    let text = " 2 * 2 ";
    let mut interpreter = Interpreter::new_with_text(text);
    assert_eq!(
        "4",
        interpreter.interpret()
    )
}

#[test]
fn test_div() {
    let text = " 2 / 2 ";
    let mut interpreter = Interpreter::new_with_text(text);
    assert_eq!(
        "1",
        interpreter.interpret()
    )
}

#[test]
fn test_long() {
    let text = " 2 + 2 * 2 - 4 / 2 ";
    let mut interpreter = Interpreter::new_with_text(text);
    assert_eq!(
        "4",
        interpreter.interpret()
    )
}

#[test]
fn test_ops() {
    fn test_add_complex() {
        let text = "2 + 3i";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("2 + 3i", interpreter.interpret());
    }

    fn test_add_rational() {
        let text = "2 + 3//4";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("11 / 4", interpreter.interpret());
    }

    fn test_add_real() {
        let text = "3//4 + 2.5";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("3.25", interpreter.interpret());
    }

    fn test_add_rationals() {
        let text = "3//8 + 5//8";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("1 / 1", interpreter.interpret());
    }

    fn test_add_rational_complex() {
        let text = "3//4 + 2i";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("0.75 + 2i", interpreter.interpret());
    }

    fn test_sub_complex() {
        let text = "3i - 2";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("-2 + 3i", interpreter.interpret());
    }

    fn test_sub_rational() {
        let text = "2 - 3//4";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("5 / 4", interpreter.interpret());
    }

    fn test_sub_real() {
        let text = "2.5 - 3//4";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("1.75", interpreter.interpret());
    }

    fn test_sub_rationals() {
        let text = "3//8 - 5//8";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("-1 / 4", interpreter.interpret());
    }

    fn test_sub_rational_complex() {
        let text = "3//4 - 2i";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("0.75 - 2i", interpreter.interpret());
    }

    fn test_mul_complex() {
        let text = "(2 + 3i) * (2 - 3i)";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("13 + 0i", interpreter.interpret());
    }

    fn test_mul_rational() {
        let text = "2//3 * 3//4";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("1 / 2", interpreter.interpret());
    }

    fn test_mul_real() {
        let text = "2.5 * 3//4";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("1.875", interpreter.interpret());
    }

    fn test_mul_rational_complex() {
        let text = "3//4 * (1 + 2i)";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("0.75 + 1.5i", interpreter.interpret());
    }

    fn test_div_complex() {
        let text = "(1 + 3i) / 2";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("0.5 + 1.5i", interpreter.interpret());
    }

    fn test_div_rational() {
        let text = "2 / 3//4";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("8 / 3", interpreter.interpret());
    }

    fn test_div_real() {
        let text = "2.5 / 1//4";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("10", interpreter.interpret());
    }

    fn test_div_rationals() {
        let text = "3//8 / 5//8";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("3 / 5", interpreter.interpret());
    }

    fn test_div_rational_complex() {
        let text = "(2 + 1i) / 1//4";
        let mut interpreter = Interpreter::new_with_text(text);
        assert_eq!("8 + 4i", interpreter.interpret());
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
    let mut interpreter = Interpreter::new_with_text(text);
    assert_eq!("2.5 - 3.2i", interpreter.interpret())
}

#[test]
fn test_parentheses() {
    let text = " (2 + 2) ";
    let mut interpreter = Interpreter::new_with_text(text);
    assert_eq!("4", interpreter.interpret())
}

#[test]
fn test_priority() {
    let text = " 2 + 2 * 2 ";
    let mut interpreter = Interpreter::new_with_text(text);
    assert_eq!("6", interpreter.interpret());

    let text = " 2 + (2 * 2) ";
    let mut interpreter = Interpreter::new_with_text(text);
    assert_eq!("6", interpreter.interpret());

    let text = " (2 + 2) * 2 ";
    let mut interpreter = Interpreter::new_with_text(text);
    assert_eq!("8", interpreter.interpret());
}

#[test]
fn test_unary() {
    let text = " -2 ";
    let mut interpreter = Interpreter::new_with_text(text);
    assert_eq!("-2", interpreter.interpret());

    let text = " +2 ";
    let mut interpreter = Interpreter::new_with_text(text);
    assert_eq!("2", interpreter.interpret());

    let text = " --2 ";
    let mut interpreter = Interpreter::new_with_text(text);
    assert_eq!("2", interpreter.interpret());
}

#[test]
fn test_multiline() {
    let text = "2 + 2";
    let mut interpreter = Interpreter::new_with_text(text);
    assert_eq!("4", interpreter.interpret());
}

#[test]
fn test_function() {
    let text = "fn hello() { 1 }";
    let mut interpreter = Interpreter::new_with_text(text);
    let func = interpreter.parser.line();
    assert_eq!(1, interpreter.parser.functions.len());
    if let Node::Function { name, .. } = func {
        assert_eq!("hello", name);
    } else {
        panic!("can't parse function");
    }
}

#[test]
fn test_function_with_arguments() {
    let text = "fn hello(num: integer) -> integer { num + 1 }";
    let mut interpreter = Interpreter::new_with_text(text);
    let func = interpreter.parser.line();
    assert_eq!(1, interpreter.parser.functions.len());
    if let Node::Function {
        name,
        arguments,
        types,
        body,
    } = func
    {
        assert_eq!("hello", name);
        assert_eq!(vec![("num".to_string(), "integer".to_string())], arguments);
        assert_eq!(vec!["integer"], types);
        println!("{:?}", body);
    } else {
        panic!("can't parse function");
    }
}
