use actix_cors::Cors;
use actix_web::http::header;

pub fn cors() -> Cors {
    Cors::default()
        //**  .allowed_origin(*) not work for allowed all origin, if you want Allows all origins dynamically use this .allowed_origin_fn(|_,_| true)
        // .allowed_origin_fn(|origin,_req_head|{           // if you want check origin use allowed_origin_fn()
        //     origin.as_bytes().ends_with(b".localhost")  // you can add your logic in the scope
        // })
        .allowed_origin("http://localhost:4200")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .max_age(3600) // 1 hour
}