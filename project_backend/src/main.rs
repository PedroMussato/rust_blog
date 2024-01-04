use axum::Router;
use axum::routing::{get, post, put, delete};
use tokio::net::TcpListener;

use project_bakcend::{
    register,
    login,
    logout,
    redefine_password,
    request_reset_password,
    reset_password,
    profile,
    update_profile,
    request_user_deletation,
    delete_user
};

#[tokio::main]
async fn main() {

    

    let routes = Router::<()>::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/logout", delete(logout))
        .route("/auth/redefine_password", put(redefine_password))
        .route("/auth/request_reset_password", post(request_reset_password))
        .route("/auth/reset_password", put(reset_password))
        .route("/auth/user_profile", get(profile))
        .route("/auth/user_profile", put(update_profile))
        .route("/auth/request_user_deletation", post(request_user_deletation))
        .route("/auth/delete_user", delete(delete_user));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
	println!("->> LISTENING on {:?}\n", listener.local_addr());
	axum::serve(listener, routes.into_make_service())
		.await
		.unwrap();
}
