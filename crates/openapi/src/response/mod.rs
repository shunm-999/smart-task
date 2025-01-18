pub mod project;
pub mod tag;
pub mod task;

#[macro_export]
macro_rules! create_response {
    ($name:ident) => {
        paste::paste! {
            // 動的に構造体を定義
            pub struct [<Api $name>]($name);

            // From トレイトの実装
            impl From<[<Api $name>]> for actix_web::HttpResponse {
                fn from(value: [<Api $name>]) -> Self {
                    let value = value.0;

                    match value {
                        $name::Status201_TheRequestHasSucceededAndANewResourceHasBeenCreatedAsAResult(res) => {
                            actix_web::HttpResponse::Created().json(res)
                        }
                        $name::Status400_TheServerCouldNotUnderstandTheRequestDueToInvalidSyntax(res) => {
                            actix_web::HttpResponse::BadRequest().json(res)
                        }
                        $name::Status401_AccessIsUnauthorized(res) => {
                            actix_web::HttpResponse::Unauthorized().json(res)
                        }
                        $name::Status403_AccessIsForbidden(res) => {
                            actix_web::HttpResponse::Forbidden().json(res)
                        }
                        $name::Status404_TheServerCannotFindTheRequestedResource(res) => {
                            actix_web::HttpResponse::NotFound().json(res)
                        }
                        $name::Status429_ClientError(res) => {
                            actix_web::HttpResponse::TooManyRequests().json(res)
                        }
                        $name::Status500_ServerError(res) => {
                            actix_web::HttpResponse::InternalServerError().json(res)
                        }
                        $name::Status503_ServiceUnavailable(res) => {
                            actix_web::HttpResponse::ServiceUnavailable().json(res)
                        }
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! ok_response {
    ($name:ident) => {
        paste::paste! {
            // 動的に構造体を定義
            pub struct [<Api $name>]($name);

            // From トレイトの実装
            impl From<[<Api $name>]> for actix_web::HttpResponse {
                fn from(value: [<Api $name>]) -> Self {
                    let value = value.0;

                    match value {
                        $name::Status200_TheRequestHasSucceeded(res) => {
                            actix_web::HttpResponse::Created().json(res)
                        }
                        $name::Status400_TheServerCouldNotUnderstandTheRequestDueToInvalidSyntax(res) => {
                            actix_web::HttpResponse::BadRequest().json(res)
                        }
                        $name::Status401_AccessIsUnauthorized(res) => {
                            actix_web::HttpResponse::Unauthorized().json(res)
                        }
                        $name::Status403_AccessIsForbidden(res) => {
                            actix_web::HttpResponse::Forbidden().json(res)
                        }
                        $name::Status404_TheServerCannotFindTheRequestedResource(res) => {
                            actix_web::HttpResponse::NotFound().json(res)
                        }
                        $name::Status429_ClientError(res) => {
                            actix_web::HttpResponse::TooManyRequests().json(res)
                        }
                        $name::Status500_ServerError(res) => {
                            actix_web::HttpResponse::InternalServerError().json(res)
                        }
                        $name::Status503_ServiceUnavailable(res) => {
                            actix_web::HttpResponse::ServiceUnavailable().json(res)
                        }
                    }
                }
            }
        }
    };
}
