#[macro_export]
macro_rules! impl_view_return  {
    ($(($name:ident => $return:ty, $fn_name:expr, $deser_fn:ident)),* $(,)?) => {
        $(pub struct $name<'a>(ViewTransaction<'a>);
        impl<'a> $name<'a> {
            pub(crate) fn view(contract: &'a Contract) -> Self {
                Self(contract.near_view(&$fn_name))
            }
            pub(crate) fn args(mut self, args: Vec<u8>) -> Self {
                self.0 = self.0.args(args);
                self
            }
            pub(crate) fn args_json<S: serde::Serialize>(mut self, args: S) -> Self {
                self.0 = self.0.args_json(args);
                self
            }
            pub(crate) fn args_borsh<B: borsh::BorshSerialize>(mut self, args: B) -> Self {
                self.0 = self.0.args_borsh(args);
                self
            }
            pub async fn transact(self)  -> anyhow::Result<ViewResult<$return>> {
                ViewResult::$deser_fn(self.0.transact().await?)
            }
        })*
    };
}

#[macro_export]
macro_rules! impl_call_return  {
    ($(($name:ident => $return:ty, $fn_name:expr, $deser_fn:ident)),* $(,)?) => {
        $(pub struct $name<'a>(CallTransaction<'a>);
        impl<'a> $name<'a> {
            pub(crate) fn call(contract: &'a Contract) -> Self {
                Self(contract.near_call(&$fn_name))
            }
            pub fn gas(mut self, gas: u64) -> Self {
                self.0 = self.0.gas(gas);
                self
            }
            pub fn max_gas(mut self) -> Self {
                self.0 = self.0.max_gas();
                self
            }
            pub fn deposit(mut self, deposit: u128) -> Self {
                self.0 = self.0.deposit(deposit);
                self
            }
            pub(crate) fn args(mut self, args: Vec<u8>) -> Self {
                self.0 = self.0.args(args);
                self
            }
            pub(crate) fn args_json<S: serde::Serialize>(mut self, args: S) -> Self {
                self.0 = self.0.args_json(args);
                self
            }
            pub(crate) fn args_borsh<B: borsh::BorshSerialize>(mut self, args: B) -> Self {
                self.0 = self.0.args_borsh(args);
                self
            }
            pub async fn transact(self) -> anyhow::Result<ExecutionResult<$return>> {
                ExecutionResult::$deser_fn(self.0.transact().await?)
            }
        })*
    };
    ($(($name:ident, $fn_name:expr)),* $(,)?) => {
        $(pub struct $name<'a>(CallTransaction<'a>);
        impl<'a> $name<'a> {
            pub(crate) fn call(contract: &'a Contract) -> Self {
                Self(contract.near_call(&$fn_name))
            }
            pub fn gas(mut self, gas: u64) -> Self {
                self.0 = self.0.gas(gas);
                self
            }
            pub fn max_gas(mut self) -> Self {
                self.0 = self.0.max_gas();
                self
            }
            pub fn deposit(mut self, deposit: u128) -> Self {
                self.0 = self.0.deposit(deposit);
                self
            }
            pub(crate) fn args(mut self, args: Vec<u8>) -> Self {
                self.0 = self.0.args(args);
                self
            }
            pub(crate) fn args_json<S: serde::Serialize>(mut self, args: S) -> Self {
                self.0 = self.0.args_json(args);
                self

            }
            pub(crate) fn args_borsh<B: borsh::BorshSerialize>(mut self, args: B) -> Self {
                self.0 = self.0.args_borsh(args);
                self
            }
            pub async fn transact(self) -> anyhow::Result<ExecutionResult<()>> {
                let result = self.0.transact().await?;
                let success = result.is_success();
                let inner = result.into_result()?;
                Ok(ExecutionResult::new(inner, (), success))
            }
        })*
    };
}
