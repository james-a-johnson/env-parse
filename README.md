# env-parse
Parse integer variables defined in the environment into constants in a rust program.

Originally I wanted this to be a way to parse anything from an environment variable into any
type that implements the `FromStr` trait. However, rust doesn't support that kind of computation
at the moment yet. Either const time evaluation would need to become much more powerful or
the proc macro would need to run in a context in which it could access the implementation of a
type's `FromStr` trait.