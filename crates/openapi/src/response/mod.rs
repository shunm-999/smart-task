mod tag;

#[macro_export]
macro_rules! create_response {
    ($name:ident) => {
        paste::paste! {
            // 動的に構造体を定義
            pub struct [<Api $name>]($name);

            // From トレイトの実装
            impl From<[<Api $name>]> for HttpResponse {
                fn from(value: [<Api $name>]) -> Self {
                    let value = value.0;

                    match value {
                        $name::Status201_TheRequestHasSucceededAndANewResourceHasBeenCreatedAsAResult(res) => {
                            HttpResponse::Created().json(res)
                        }
                        $name::Status400_TheServerCouldNotUnderstandTheRequestDueToInvalidSyntax(res) => {
                            HttpResponse::BadRequest().json(res)
                        }
                        $name::Status401_AccessIsUnauthorized(res) => {
                            HttpResponse::Unauthorized().json(res)
                        }
                        $name::Status403_AccessIsForbidden(res) => {
                            HttpResponse::Forbidden().json(res)
                        }
                        $name::Status404_TheServerCannotFindTheRequestedResource(res) => {
                            HttpResponse::NotFound().json(res)
                        }
                        $name::Status429_ClientError(res) => {
                            HttpResponse::TooManyRequests().json(res)
                        }
                        $name::Status500_ServerError(res) => {
                            HttpResponse::InternalServerError().json(res)
                        }
                        $name::Status503_ServiceUnavailable(res) => {
                            HttpResponse::ServiceUnavailable().json(res)
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
            impl From<[<Api $name>]> for HttpResponse {
                fn from(value: [<Api $name>]) -> Self {
                    let value = value.0;

                    match value {
                        $name::Status200_TheRequestHasSucceeded(res) => {
                            HttpResponse::Created().json(res)
                        }
                        $name::Status400_TheServerCouldNotUnderstandTheRequestDueToInvalidSyntax(res) => {
                            HttpResponse::BadRequest().json(res)
                        }
                        $name::Status401_AccessIsUnauthorized(res) => {
                            HttpResponse::Unauthorized().json(res)
                        }
                        $name::Status403_AccessIsForbidden(res) => {
                            HttpResponse::Forbidden().json(res)
                        }
                        $name::Status404_TheServerCannotFindTheRequestedResource(res) => {
                            HttpResponse::NotFound().json(res)
                        }
                        $name::Status429_ClientError(res) => {
                            HttpResponse::TooManyRequests().json(res)
                        }
                        $name::Status500_ServerError(res) => {
                            HttpResponse::InternalServerError().json(res)
                        }
                        $name::Status503_ServiceUnavailable(res) => {
                            HttpResponse::ServiceUnavailable().json(res)
                        }
                    }
                }
            }
        }
    };
}
