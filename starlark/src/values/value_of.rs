/*
 * Copyright 2018 The Starlark in Rust Authors.
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

use std::ops::Deref;

use dupe::Dupe;

use crate::typing::Ty;
use crate::values::type_repr::StarlarkTypeRepr;
use crate::values::AllocValue;
use crate::values::Heap;
use crate::values::UnpackValue;
use crate::values::Value;

/// A wrapper that keeps the original value on the heap for use elsewhere,
/// and also, when unpacked, unpacks the value to validate it is of
/// the correct type. Has an [`UnpackValue`] instance, so often used as
/// an argument to [`#[starlark_module]`](macro@crate::starlark_module) defined
/// functions.
///
/// Two container specializations of this are [`ListOf`](crate::values::list::ListOf)
/// and [`DictOf`](crate::values::dict::DictOf), which
/// validate the types of their containers on unpack, but do not store the
/// resulting Vec/Map
#[derive(Debug, Copy, Clone, Dupe)]
pub struct ValueOf<'v, T: UnpackValue<'v>> {
    /// The original [`Value`] on the same heap.
    pub value: Value<'v>,
    /// The value that was unpacked.
    pub typed: T,
}

impl<'v, T: UnpackValue<'v>> Deref for ValueOf<'v, T> {
    type Target = Value<'v>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'v, T: UnpackValue<'v>> StarlarkTypeRepr for ValueOf<'v, T> {
    fn starlark_type_repr() -> Ty {
        T::starlark_type_repr()
    }
}

impl<'v, T: UnpackValue<'v>> UnpackValue<'v> for ValueOf<'v, T> {
    fn expected() -> String {
        T::expected()
    }

    fn unpack_value(value: Value<'v>) -> Option<Self> {
        let typed = T::unpack_value(value)?;
        Some(Self { value, typed })
    }
}

impl<'v, T: UnpackValue<'v>> AllocValue<'v> for ValueOf<'v, T> {
    fn alloc_value(self, _heap: &'v Heap) -> Value<'v> {
        self.value
    }
}
