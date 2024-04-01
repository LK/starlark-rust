/*
 * Copyright 2019 The Starlark in Rust Authors.
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Tests for functions, callables and calls.

use crate::typing::tests::TypeCheck;

#[test]
fn test_type_kwargs() {
    TypeCheck::new().check(
        "type_kwargs",
        r#"
def foo(**kwargs):
    pass

def bar():
    foo(**{1: "x"})
"#,
    );
}

#[test]
fn test_types_of_args_kwargs() {
    TypeCheck::new().ty("args").ty("kwargs").check(
        "types_of_args_kwargs",
        r#"
def foo(*args: str, **kwargs: int):
    pass

def test():
    # Good
    foo("a")
    foo(b=1)
    # Bad
    foo(1)
    foo(c="x")
"#,
    );
}

#[test]
fn test_kwargs_in_native_code() {
    TypeCheck::new().check(
        "kwargs_in_native_code",
        r#"
def test():
    # Good.
    accepts_typed_kwargs(x=1)
    # Bad.
    accepts_typed_kwargs(x=None)
"#,
    );
}

#[test]
fn test_call_callable() {
    TypeCheck::new().check(
        "call_callable",
        r#"
def foo(x: typing.Callable):
    x()
"#,
    );
}

#[test]
fn test_call_not_callable() {
    TypeCheck::new().check(
        "call_not_callable",
        r#"
def foo(x: list):
    x()
"#,
    );
}

#[test]
fn test_call_callable_or_not_callable() {
    TypeCheck::new().check(
        "call_callable_or_not_callable",
        r#"
def foo(x: [typing.Callable, str], y: [str, typing.Callable]):
    x()
    y()
"#,
    );
}

#[test]
fn test_callable_with_args() {
    TypeCheck::new().check(
        "callable_with_args",
        r#"
def accept_f(x: typing.Callable[[int, str], str]):
    pass

def good_function(x: int, y: str) -> str:
    return ""

def bad_function(x: int, y: bool) -> str:
    return ""

def test():
    accept_f(good_function)
    accept_f(bad_function)
"#,
    );
}

#[test]
fn test_calls() {
    TypeCheck::new().check(
        "calls",
        r#"
def f(y): pass

def g():
    # Extra parameter.
    f(1, 2)

    # Not enough parameters.
    f()
"#,
    );
}

#[test]
fn test_never_call_bug() {
    TypeCheck::new().ty("y").check(
        "never_call_bug",
        r#"
def foo(x: typing.Never):
    y = x(1)
"#,
    );
}
