use erg_common::vis::Visibility;

use erg_type::constructors::func1;
use erg_type::Type;
use Type::*;

use crate::context::Context;
use crate::varinfo::Mutability;
use Mutability::*;
use Visibility::*;

impl Context {
    pub(crate) fn init_py_math_mod() -> Self {
        let mut math = Context::module("math".into(), 10);
        math.register_impl("pi", Float, Immutable, Public);
        math.register_impl("tau", Float, Immutable, Public);
        math.register_impl("e", Float, Immutable, Public);
        math.register_impl("sin", func1(Float, Float), Immutable, Public);
        math.register_impl("cos", func1(Float, Float), Immutable, Public);
        math.register_impl("tan", func1(Float, Float), Immutable, Public);
        math
    }
}