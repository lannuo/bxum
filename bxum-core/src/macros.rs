#[cfg(feature = "tracing")]
#[doc(hidden)]
#[macro_export]
macro_rules! __log_rejection {
    (
        rejection_type = $ty:ident,
        body_text = $body_text:expr,
        status = $status:expr,
    ) => {
        {
            $crate::__private::tracing::event!(
                target: "bxum::rejection",
                $crate::__private::tracing::Level::TRACE,
                status = $status.as_u16(),
                body = $body_text,
                rejection_type = ::std::any::type_name::<$ty>(),
                "rejecting request",
            )
        }
    };
}

#[cfg(not(feature = "tracing"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __log_rejection {
    (
        rejection_type = $ty:ident,
        body_text = $body_text:expr,
        status = $status:expr,
    ) => {};
}

