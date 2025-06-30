use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use axum::{Router, Json, routing::{get, post}, http::StatusCode, response::IntoResponse};
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: T,
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    error: String,
}

#[derive(Serialize)]
struct KeypairData {
    pubkey: String,
    secret: String,
}

#[derive(Deserialize)]
struct CreateTokenRequest {
    #[serde(rename = "mintAuthority")]
    mint_authority: String,
    mint: String,
    decimals: u8,
}

#[derive(Serialize)]
struct AccountMeta {
    pubkey: String,
    is_signer: bool,
    is_writable: bool,
}

#[derive(Serialize)]
struct TokenInstructionData {
    program_id: String,
    accounts: Vec<AccountMeta>,
    instruction_data: String,
}

#[derive(Serialize)]
struct MessageData {
    message: String,
}


fn create_error_response(error_message: &str) -> impl IntoResponse {
    let error_response = ErrorResponse {
        success: false,
        error: error_message.to_string(),
    };
    (StatusCode::BAD_REQUEST, Json(error_response))
}

async fn root_handler() -> Json<ApiResponse<MessageData>> {
    let response = ApiResponse {
         success: true,
        data: MessageData {
            message: "gm gm".to_string(),
        },
    };
    Json(response)
}

async fn keypair_handler() -> impl IntoResponse {
    let keypair = Keypair::new();

    let pubkey = keypair.pubkey().to_string(); 
    let secret = bs58::encode(keypair.to_bytes()).into_string(); 

    let response = ApiResponse {
        success: true,
        data: KeypairData {
            pubkey,
            secret,
        },
    };

    (StatusCode::OK, Json(response))
}

async fn create_token_handler(Json(payload): Json<CreateTokenRequest>) -> impl IntoResponse {
    let accounts = vec![
        AccountMeta {
            pubkey: payload.mint.clone(),
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: payload.mint_authority.clone(),
            is_signer: true,
            is_writable: false,
        },
    ];

    let instruction_data = TokenInstructionData {
        program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
        accounts,
        instruction_data: base64::encode(&[0, payload.decimals]),
    };

    let response = ApiResponse {
        success: true,
        data: instruction_data,
    };

    (StatusCode::OK, Json(response))
}
#[tokio::main]
async fn main() {

    let app = Router::new()
    .route("/", get(root_handler))
    .route("/keypair", get(keypair_handler))
    .route("/token/create", post(create_token_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    
    println!("Server is running on http://{}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}