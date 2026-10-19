//! Call context carried by every client error (issue #345).
//!
//! An error by itself does not say which call produced it or with what
//! arguments, which makes debugging a failing integration harder than it
//! needs to be once an application calls several registry methods from the
//! same code path. Every error either client returns is therefore wrapped
//! with the failing method's name and a debug representation of its
//! **non-secret** arguments, so a caller's own error log is self-explanatory
//! without separately logging the call site.
//!
//! Secret hygiene (the same concern as the keeper-bot's `requireEnv` secret
//! flag): the context only ever records the *contract arguments* of a call —
//! the signer is configuration, not an argument, and is never formatted into
//! an error. Signer implementations that hold key material should
//! additionally wrap it in [`Redacted`] so even their own `Debug` output
//! cannot leak it.

use core::fmt;

/// Which call failed, and the debug form of its non-secret arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallContext {
    /// The client method / contract function name.
    pub method: &'static str,
    /// Debug representations of the call's arguments, in call order. Only
    /// contract arguments appear here; keys, seeds and signatures never do.
    pub args: std::vec::Vec<String>,
}

impl CallContext {
    pub fn new(method: &'static str, args: std::vec::Vec<String>) -> Self {
        Self { method, args }
    }

    /// Context from anything debug-printable — the RPC client's `ScVal`
    /// argument lists use this.
    pub fn from_debug<A: fmt::Debug>(method: &'static str, args: &[A]) -> Self {
        Self {
            method,
            args: args.iter().map(|a| format!("{a:?}")).collect(),
        }
    }
}

impl fmt::Display for CallContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.method, self.args.join(", "))
    }
}

/// Wrapper whose `Debug` and `Display` print `<redacted>` instead of the
/// value. Hold signing keys, seeds and other secrets in this inside signer
/// implementations, so no derived `Debug` — including a future one on an
/// error path — can ever print the material itself.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Redacted<T>(pub T);

impl<T> Redacted<T> {
    /// The wrapped secret. Deliberately a method rather than `Deref`, so
    /// every use of the material is an explicit, greppable call.
    pub fn expose(&self) -> &T {
        &self.0
    }
}

impl<T> fmt::Debug for Redacted<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<redacted>")
    }
}

impl<T> fmt::Display for Redacted<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<redacted>")
    }
}

/// Builds the `Vec<String>` of argument debug forms for a [`CallContext`].
#[macro_export]
macro_rules! call_args {
    () => { std::vec::Vec::<String>::new() };
    ($($arg:expr),+ $(,)?) => { std::vec![$(format!("{:?}", $arg)),+] };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_formats_as_a_call_site() {
        let ctx = CallContext::new("set_fee_bps", vec!["Address(..)".into(), "250".into()]);
        assert_eq!(ctx.to_string(), "set_fee_bps(Address(..), 250)");
    }

    #[test]
    fn redacted_never_prints_the_value_it_holds() {
        let secret = Redacted([0xABu8; 32]);
        assert_eq!(format!("{secret:?}"), "<redacted>");
        assert_eq!(format!("{secret}"), "<redacted>");
        assert_eq!(secret.expose()[0], 0xAB);
    }
}
