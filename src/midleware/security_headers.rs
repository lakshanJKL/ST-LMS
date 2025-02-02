use actix_web::middleware::DefaultHeaders;
use actix_web::http::header;
use actix_web::web::head;

pub fn security_headers() -> DefaultHeaders {
    DefaultHeaders::new()
        // Enforce HTTPS with HSTS
        .header(header::STRICT_TRANSPORT_SECURITY,
                "max-age=31536000; includeSubDomains; preload",
        )
        // Prevent click jacking
        .header(header::X_FRAME_OPTIONS,
                "DENY",
        )
        // Prevent MIME type sniffing
        .header(header::X_CONTENT_TYPE_OPTIONS,
                "nosniff",
        )
        // Control content sources
        .header(
            header::CONTENT_SECURITY_POLICY,
            "default-src 'self'; script-src 'self'; object-src 'none'; frame-ancestors 'none';",
        )
        // Protect privacy by controlling referrer data
        .header(header::REFERRER_POLICY,
                "strict-origin-when-cross-origin",
        )
        // Restrict access to browser features
        .header(
            header::PERMISSIONS_POLICY,
            "geolocation=(), camera=(), microphone=(), payment=(), usb=()",
        )
        // Protect against cross-site scripting (XSS)
        .header(header::X_XSS_PROTECTION,
                "0",
        )
        // Prevent cache storage of sensitive data
        .header(header::CACHE_CONTROL,
                "no-store, no-cache, must-revalidate, proxy-revalidate",
        )
        .header(header::PRAGMA,
                "no-cache",
        )
        .header(header::EXPIRES,
                "0",
        )
        // Optionally remove server signature (Actix exposes this by default)
        .header(header::SERVER, "")
}
