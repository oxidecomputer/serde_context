
pub trait SerializeWithContext/*<C: Copy>*/: serde::Serialize {
    type Context;
    fn serialize_with_context<S: serde::Serializer>(&self, serializer: S, context: Self::Context) -> core::result::Result<S::Ok, S::Error> {
        self.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
