use actix_web::middleware::Logger;

pub fn logger() ->Logger{
    Logger::new("%a %{User-Agent}i %r %s %b %Dms")
}

/*
%a: Client IP address.

%{User-Agent}i: User-Agent header.

%r: Request line (method, path, HTTP version).

%s: Response status code.

%b: Response size in bytes.

%Dms: Request duration in milliseconds.
*/