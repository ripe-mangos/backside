use core::fmt::Display;

pub type Result<T> = core::result::Result<T, Error>;

macro_rules! errs {
    ( $( $ty:ident : $str:literal ),* ) => {
        pub enum Error {
            $(
            #[doc = $str]
            $ty,
            )*
        }
        impl Display for Error {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                use Error::*;
                write!(f, "{}",
                    match self {
                        $(
                        $ty => $str,
                        )*
                    }
                )
            }
        }
    };
}

errs! {
    StructureInvalid: "basic file structure invalid",
    StyleUndefined: "style does not exist or was not defined",
    OCInvalid: "invalid override code",
    OCInvalidParams: "invalid override code parameters",
    OCMissingParams: "missing override code parameters"
}

