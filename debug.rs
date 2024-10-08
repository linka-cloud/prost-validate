mod errors {
    use thiserror::Error;
    mod string {
        use thiserror::Error;
        pub enum Error {
            #[error("value must be equal to {0}")]
            Const(String),
            #[error("value characters length must be equal to {0}")]
            Len(usize),
            #[error("value characters length must be greater than or equal to {0}")]
            MinLen(usize),
            #[error("value characters length must be less than or equal to {0}")]
            MaxLen(usize),
            #[error("value bytes length must be equal to {0}")]
            LenBytes(usize),
            #[error("value bytes length must be greater than or equal to {0}")]
            MinLenBytes(usize),
            #[error("value bytes length must be less than or equal to {0}")]
            MaxLenBytes(usize),
            #[error("value must match pattern {0}")]
            Pattern(String),
            #[error("value must have prefix {0}")]
            Prefix(String),
            #[error("value must have suffix {0}")]
            Suffix(String),
            #[error("value must contain {0}")]
            Contains(String),
            #[error("value must not contain {0}")]
            NotContains(String),
            #[error("value must be in {0:?}")]
            In(Vec<String>),
            #[error("value must not be in {0:?}")]
            NotIn(Vec<String>),
            #[error("value must be a valid email address")]
            Email,
            #[error("value must be a valid hostname")]
            Hostname,
            #[error("value must be a valid IP address")]
            Ip,
            #[error("value must be a valid IPv4 address")]
            Ipv4,
            #[error("value must be a valid IPv6 address")]
            Ipv6,
            #[error("value must be a valid URI")]
            Uri,
            #[error("value must be a valid URI ref")]
            UriRef,
            #[error("value must be a valid hostname or IP address")]
            Address,
            #[error("value must be a valid UUID")]
            Uuid,
            #[error("value must be a valid HTTP Header name")]
            HeaderName,
            #[error("value must be a valid HTTP Header value")]
            HeaderValue,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Error {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Error::Const(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Const",
                            &__self_0,
                        )
                    }
                    Error::Len(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Len",
                            &__self_0,
                        )
                    }
                    Error::MinLen(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "MinLen",
                            &__self_0,
                        )
                    }
                    Error::MaxLen(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "MaxLen",
                            &__self_0,
                        )
                    }
                    Error::LenBytes(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "LenBytes",
                            &__self_0,
                        )
                    }
                    Error::MinLenBytes(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "MinLenBytes",
                            &__self_0,
                        )
                    }
                    Error::MaxLenBytes(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "MaxLenBytes",
                            &__self_0,
                        )
                    }
                    Error::Pattern(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Pattern",
                            &__self_0,
                        )
                    }
                    Error::Prefix(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Prefix",
                            &__self_0,
                        )
                    }
                    Error::Suffix(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Suffix",
                            &__self_0,
                        )
                    }
                    Error::Contains(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Contains",
                            &__self_0,
                        )
                    }
                    Error::NotContains(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "NotContains",
                            &__self_0,
                        )
                    }
                    Error::In(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "In",
                            &__self_0,
                        )
                    }
                    Error::NotIn(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "NotIn",
                            &__self_0,
                        )
                    }
                    Error::Email => ::core::fmt::Formatter::write_str(f, "Email"),
                    Error::Hostname => ::core::fmt::Formatter::write_str(f, "Hostname"),
                    Error::Ip => ::core::fmt::Formatter::write_str(f, "Ip"),
                    Error::Ipv4 => ::core::fmt::Formatter::write_str(f, "Ipv4"),
                    Error::Ipv6 => ::core::fmt::Formatter::write_str(f, "Ipv6"),
                    Error::Uri => ::core::fmt::Formatter::write_str(f, "Uri"),
                    Error::UriRef => ::core::fmt::Formatter::write_str(f, "UriRef"),
                    Error::Address => ::core::fmt::Formatter::write_str(f, "Address"),
                    Error::Uuid => ::core::fmt::Formatter::write_str(f, "Uuid"),
                    Error::HeaderName => {
                        ::core::fmt::Formatter::write_str(f, "HeaderName")
                    }
                    Error::HeaderValue => {
                        ::core::fmt::Formatter::write_str(f, "HeaderValue")
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Error {
            #[inline]
            fn clone(&self) -> Error {
                match self {
                    Error::Const(__self_0) => {
                        Error::Const(::core::clone::Clone::clone(__self_0))
                    }
                    Error::Len(__self_0) => {
                        Error::Len(::core::clone::Clone::clone(__self_0))
                    }
                    Error::MinLen(__self_0) => {
                        Error::MinLen(::core::clone::Clone::clone(__self_0))
                    }
                    Error::MaxLen(__self_0) => {
                        Error::MaxLen(::core::clone::Clone::clone(__self_0))
                    }
                    Error::LenBytes(__self_0) => {
                        Error::LenBytes(::core::clone::Clone::clone(__self_0))
                    }
                    Error::MinLenBytes(__self_0) => {
                        Error::MinLenBytes(::core::clone::Clone::clone(__self_0))
                    }
                    Error::MaxLenBytes(__self_0) => {
                        Error::MaxLenBytes(::core::clone::Clone::clone(__self_0))
                    }
                    Error::Pattern(__self_0) => {
                        Error::Pattern(::core::clone::Clone::clone(__self_0))
                    }
                    Error::Prefix(__self_0) => {
                        Error::Prefix(::core::clone::Clone::clone(__self_0))
                    }
                    Error::Suffix(__self_0) => {
                        Error::Suffix(::core::clone::Clone::clone(__self_0))
                    }
                    Error::Contains(__self_0) => {
                        Error::Contains(::core::clone::Clone::clone(__self_0))
                    }
                    Error::NotContains(__self_0) => {
                        Error::NotContains(::core::clone::Clone::clone(__self_0))
                    }
                    Error::In(__self_0) => {
                        Error::In(::core::clone::Clone::clone(__self_0))
                    }
                    Error::NotIn(__self_0) => {
                        Error::NotIn(::core::clone::Clone::clone(__self_0))
                    }
                    Error::Email => Error::Email,
                    Error::Hostname => Error::Hostname,
                    Error::Ip => Error::Ip,
                    Error::Ipv4 => Error::Ipv4,
                    Error::Ipv6 => Error::Ipv6,
                    Error::Uri => Error::Uri,
                    Error::UriRef => Error::UriRef,
                    Error::Address => Error::Address,
                    Error::Uuid => Error::Uuid,
                    Error::HeaderName => Error::HeaderName,
                    Error::HeaderValue => Error::HeaderValue,
                }
            }
        }
        #[allow(unused_qualifications)]
        #[automatically_derived]
        impl std::error::Error for Error {}
        #[allow(unused_qualifications)]
        #[automatically_derived]
        impl ::core::fmt::Display for Error {
            fn fmt(
                &self,
                __formatter: &mut ::core::fmt::Formatter,
            ) -> ::core::fmt::Result {
                use thiserror::__private::AsDisplay as _;
                #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
                match self {
                    Error::Const(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!("value must be equal to {0}", _0.as_display()),
                            )
                    }
                    Error::Len(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!(
                                    "value characters length must be equal to {0}",
                                    _0.as_display(),
                                ),
                            )
                    }
                    Error::MinLen(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!(
                                    "value characters length must be greater than or equal to {0}",
                                    _0.as_display(),
                                ),
                            )
                    }
                    Error::MaxLen(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!(
                                    "value characters length must be less than or equal to {0}",
                                    _0.as_display(),
                                ),
                            )
                    }
                    Error::LenBytes(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!(
                                    "value bytes length must be equal to {0}",
                                    _0.as_display(),
                                ),
                            )
                    }
                    Error::MinLenBytes(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!(
                                    "value bytes length must be greater than or equal to {0}",
                                    _0.as_display(),
                                ),
                            )
                    }
                    Error::MaxLenBytes(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!(
                                    "value bytes length must be less than or equal to {0}",
                                    _0.as_display(),
                                ),
                            )
                    }
                    Error::Pattern(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!(
                                    "value must match pattern {0}",
                                    _0.as_display(),
                                ),
                            )
                    }
                    Error::Prefix(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!("value must have prefix {0}", _0.as_display()),
                            )
                    }
                    Error::Suffix(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!("value must have suffix {0}", _0.as_display()),
                            )
                    }
                    Error::Contains(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!("value must contain {0}", _0.as_display()),
                            )
                    }
                    Error::NotContains(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!("value must not contain {0}", _0.as_display()),
                            )
                    }
                    Error::In(_0) => {
                        __formatter.write_fmt(format_args!("value must be in {0:?}", _0))
                    }
                    Error::NotIn(_0) => {
                        __formatter
                            .write_fmt(format_args!("value must not be in {0:?}", _0))
                    }
                    Error::Email {} => {
                        __formatter.write_str("value must be a valid email address")
                    }
                    Error::Hostname {} => {
                        __formatter.write_str("value must be a valid hostname")
                    }
                    Error::Ip {} => {
                        __formatter.write_str("value must be a valid IP address")
                    }
                    Error::Ipv4 {} => {
                        __formatter.write_str("value must be a valid IPv4 address")
                    }
                    Error::Ipv6 {} => {
                        __formatter.write_str("value must be a valid IPv6 address")
                    }
                    Error::Uri {} => __formatter.write_str("value must be a valid URI"),
                    Error::UriRef {} => {
                        __formatter.write_str("value must be a valid URI ref")
                    }
                    Error::Address {} => {
                        __formatter
                            .write_str("value must be a valid hostname or IP address")
                    }
                    Error::Uuid {} => __formatter.write_str("value must be a valid UUID"),
                    Error::HeaderName {} => {
                        __formatter.write_str("value must be a valid HTTP Header name")
                    }
                    Error::HeaderValue {} => {
                        __formatter.write_str("value must be a valid HTTP Header value")
                    }
                }
            }
        }
    }
    mod bytes {}
    mod number {
        use thiserror::Error;
        pub enum Error<T> {
            #[error("value must be equal to {0}")]
            Const(T),
            #[error("value must be less than {0}")]
            Lt(T),
            #[error("value must be less than or equal to {0}")]
            Lte(T),
            #[error("value must be greater than {0}")]
            Gt(T),
            #[error("value must be greater than or equal to {0:?}")]
            Gte(T),
            #[error("value must be in range {0}{1}, {2}{3}")]
            Range(String, T, T, String),
            #[error("value must be in {0:?}")]
            In(Vec<T>),
            #[error("value must not be in {0:?}")]
            NotIn(Vec<T>),
        }
        #[automatically_derived]
        impl<T: ::core::fmt::Debug> ::core::fmt::Debug for Error<T> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Error::Const(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Const",
                            &__self_0,
                        )
                    }
                    Error::Lt(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Lt",
                            &__self_0,
                        )
                    }
                    Error::Lte(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Lte",
                            &__self_0,
                        )
                    }
                    Error::Gt(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Gt",
                            &__self_0,
                        )
                    }
                    Error::Gte(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Gte",
                            &__self_0,
                        )
                    }
                    Error::Range(__self_0, __self_1, __self_2, __self_3) => {
                        ::core::fmt::Formatter::debug_tuple_field4_finish(
                            f,
                            "Range",
                            __self_0,
                            __self_1,
                            __self_2,
                            &__self_3,
                        )
                    }
                    Error::In(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "In",
                            &__self_0,
                        )
                    }
                    Error::NotIn(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "NotIn",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl<T: ::core::clone::Clone> ::core::clone::Clone for Error<T> {
            #[inline]
            fn clone(&self) -> Error<T> {
                match self {
                    Error::Const(__self_0) => {
                        Error::Const(::core::clone::Clone::clone(__self_0))
                    }
                    Error::Lt(__self_0) => {
                        Error::Lt(::core::clone::Clone::clone(__self_0))
                    }
                    Error::Lte(__self_0) => {
                        Error::Lte(::core::clone::Clone::clone(__self_0))
                    }
                    Error::Gt(__self_0) => {
                        Error::Gt(::core::clone::Clone::clone(__self_0))
                    }
                    Error::Gte(__self_0) => {
                        Error::Gte(::core::clone::Clone::clone(__self_0))
                    }
                    Error::Range(__self_0, __self_1, __self_2, __self_3) => {
                        Error::Range(
                            ::core::clone::Clone::clone(__self_0),
                            ::core::clone::Clone::clone(__self_1),
                            ::core::clone::Clone::clone(__self_2),
                            ::core::clone::Clone::clone(__self_3),
                        )
                    }
                    Error::In(__self_0) => {
                        Error::In(::core::clone::Clone::clone(__self_0))
                    }
                    Error::NotIn(__self_0) => {
                        Error::NotIn(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        #[automatically_derived]
        impl<T> std::error::Error for Error<T>
        where
            Self: ::core::fmt::Debug + ::core::fmt::Display,
        {}
        #[allow(unused_qualifications)]
        #[automatically_derived]
        impl<T> ::core::fmt::Display for Error<T>
        where
            T: ::core::fmt::Display + ::core::fmt::Debug,
            Vec<T>: ::core::fmt::Debug,
        {
            fn fmt(
                &self,
                __formatter: &mut ::core::fmt::Formatter,
            ) -> ::core::fmt::Result {
                use thiserror::__private::AsDisplay as _;
                #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
                match self {
                    Error::Const(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!("value must be equal to {0}", _0.as_display()),
                            )
                    }
                    Error::Lt(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!("value must be less than {0}", _0.as_display()),
                            )
                    }
                    Error::Lte(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!(
                                    "value must be less than or equal to {0}",
                                    _0.as_display(),
                                ),
                            )
                    }
                    Error::Gt(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!(
                                    "value must be greater than {0}",
                                    _0.as_display(),
                                ),
                            )
                    }
                    Error::Gte(_0) => {
                        __formatter
                            .write_fmt(
                                format_args!(
                                    "value must be greater than or equal to {0:?}",
                                    _0,
                                ),
                            )
                    }
                    Error::Range(_0, _1, _2, _3) => {
                        __formatter
                            .write_fmt(
                                format_args!(
                                    "value must be in range {0}{1}, {2}{3}",
                                    _0.as_display(),
                                    _1.as_display(),
                                    _2.as_display(),
                                    _3.as_display(),
                                ),
                            )
                    }
                    Error::In(_0) => {
                        __formatter.write_fmt(format_args!("value must be in {0:?}", _0))
                    }
                    Error::NotIn(_0) => {
                        __formatter
                            .write_fmt(format_args!("value must not be in {0:?}", _0))
                    }
                }
            }
        }
    }
    mod list {}
    mod map {}
    mod duration {}
    mod timestamp {}
    mod message {}
    pub enum Error {
        String(#[source] string::Error),
        Int32(#[source] number::Error<i32>),
        Int64(#[source] number::Error<i64>),
        Uint32(#[source] number::Error<u32>),
        Uint64(#[source] number::Error<u64>),
        Sint32(#[source] number::Error<i32>),
        Sint64(#[source] number::Error<i64>),
        Fixed32(#[source] number::Error<u32>),
        Fixed64(#[source] number::Error<u64>),
        Sfixed32(#[source] number::Error<i32>),
        Sfixed64(#[source] number::Error<i64>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Error {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Error::String(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "String",
                        &__self_0,
                    )
                }
                Error::Int32(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Int32",
                        &__self_0,
                    )
                }
                Error::Int64(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Int64",
                        &__self_0,
                    )
                }
                Error::Uint32(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Uint32",
                        &__self_0,
                    )
                }
                Error::Uint64(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Uint64",
                        &__self_0,
                    )
                }
                Error::Sint32(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Sint32",
                        &__self_0,
                    )
                }
                Error::Sint64(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Sint64",
                        &__self_0,
                    )
                }
                Error::Fixed32(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Fixed32",
                        &__self_0,
                    )
                }
                Error::Fixed64(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Fixed64",
                        &__self_0,
                    )
                }
                Error::Sfixed32(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Sfixed32",
                        &__self_0,
                    )
                }
                Error::Sfixed64(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Sfixed64",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Error {
        #[inline]
        fn clone(&self) -> Error {
            match self {
                Error::String(__self_0) => {
                    Error::String(::core::clone::Clone::clone(__self_0))
                }
                Error::Int32(__self_0) => {
                    Error::Int32(::core::clone::Clone::clone(__self_0))
                }
                Error::Int64(__self_0) => {
                    Error::Int64(::core::clone::Clone::clone(__self_0))
                }
                Error::Uint32(__self_0) => {
                    Error::Uint32(::core::clone::Clone::clone(__self_0))
                }
                Error::Uint64(__self_0) => {
                    Error::Uint64(::core::clone::Clone::clone(__self_0))
                }
                Error::Sint32(__self_0) => {
                    Error::Sint32(::core::clone::Clone::clone(__self_0))
                }
                Error::Sint64(__self_0) => {
                    Error::Sint64(::core::clone::Clone::clone(__self_0))
                }
                Error::Fixed32(__self_0) => {
                    Error::Fixed32(::core::clone::Clone::clone(__self_0))
                }
                Error::Fixed64(__self_0) => {
                    Error::Fixed64(::core::clone::Clone::clone(__self_0))
                }
                Error::Sfixed32(__self_0) => {
                    Error::Sfixed32(::core::clone::Clone::clone(__self_0))
                }
                Error::Sfixed64(__self_0) => {
                    Error::Sfixed64(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl std::error::Error for Error {
        fn source(&self) -> ::core::option::Option<&(dyn std::error::Error + 'static)> {
            use thiserror::__private::AsDynError as _;
            #[allow(deprecated)]
            match self {
                Error::String { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                Error::Int32 { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                Error::Int64 { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                Error::Uint32 { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                Error::Uint64 { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                Error::Sint32 { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                Error::Sint64 { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                Error::Fixed32 { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                Error::Fixed64 { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                Error::Sfixed32 { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                Error::Sfixed64 { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
            }
        }
    }
}
