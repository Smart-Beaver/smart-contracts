#[macro_export]
macro_rules! inject_test_utils {
    () => {

    use ink::codegen::Env;

    impl Token {
        //Very basic logic behind handling mint and burn. It is left here to keep backward compatibility with existing tests
        //@FIXME it should be removed in the future
        #[cfg(feature = "test-only")]
        pub fn burn_test(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            let events = self.data.burn(self.env().caller(), account, id)?;
            self.emit_events(events);
            Ok(())
        }

        #[cfg(feature = "test-only")]
        pub fn mint_test(&mut self, id: Id) -> Result<(), PSP34Error> {
            let events = self.data.mint(self.env().caller(), id)?;
            self.emit_events(events);
            Ok(())
        }
    }

    };
}