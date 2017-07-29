use std::fmt::{self, Display};
use std::marker::PhantomData;
use std::mem;

use backend::Backend;
use super::{QueryFragment, AstPass};

#[allow(missing_debug_implementations)]
/// A struct that implements `fmt::Display` by walking the given AST and
/// writing the `fmt::Debug` implementation of each bind parameter.
pub struct DebugBinds<'a, T: 'a, DB> {
    query: &'a T,
    _marker: PhantomData<DB>,
}

impl<'a, T, DB> DebugBinds<'a, T, DB> {
    pub fn new(query: &'a T) -> Self {
        DebugBinds {
            query,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, DB> Display for DebugBinds<'a, T, DB> where
    DB: Backend,
    T: QueryFragment<DB>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("binds: ")?;
        let mut list = f.debug_list();
        {
            // Safe because the lifetime is shortened to one smaller
            // than the lifetime of the formatter.
            let list_with_shorter_lifetime = unsafe { mem::transmute(&mut list) };
            let ast_pass = AstPass::debug_binds(list_with_shorter_lifetime);
            self.query.walk_ast(ast_pass).map_err(|_| fmt::Error)?;
        }
        list.finish()?;
        Ok(())
    }
}
