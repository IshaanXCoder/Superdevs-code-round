use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use axum::{Router, Json, routing::{get, post}, http::StatusCode, response::IntoResponse};
use serde::{Serialize, Deserialize};
use base64::Engine;
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

#[derive(Serialize)]
struct MessageData {
    message: String,
}

#[derive(Serialize)]
struct AccountMeta {
    pubkey: String,
    is_signer: bool,
    is_writable: bool,
}

#[derive(Serialize)]
struct InstructionData {
    program_id: String,
    accounts: Vec<AccountMeta>,
    instruction_data: String,
}

#[derive(Serialize)]
struct SignatureData {
    signature: String,
    public_key: String,
    message: String,
}

#[derive(Deserialize)]
struct CreateTokenRequest {
    #[serde(rename = "mintAuthority")]
    mint_authority: String,
    mint: String,
    decimals: u8,
}

#[derive(Deserialize)]
struct MintTokenRequest {
    mint: String,
    destination: String,
    authority: String,
    amount: u64,
}

#[derive(Deserialize)]
struct SignMessageRequest {
    message: String,
    secret: String,
}

fn error_response(message: &str) -> impl IntoResponse {
    let response = ErrorResponse {
        success: false,
        error: message.to_string(),
    };
    (StatusCode::BAD_REQUEST, Json(response))
}

async fn root_handler() -> impl IntoResponse {
    let response = ApiResponse {
        success: true,
        data: MessageData {
            message: "gm gm".to_string(),
        },
    };
    (StatusCode::OK, Json(response))
}

async fn keypair_handler() -> impl IntoResponse {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string(); 
    let secret = bs58::encode(keypair.to_bytes()).into_string(); 

    let response = ApiResponse {
        success: true,
        data: KeypairData { pubkey, secret },
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

    let instruction_data = InstructionData {
        program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
        accounts,
        instruction_data: base64::engine::general_purpose::STANDARD.encode(&[0, payload.decimals]),
    };

    let response = ApiResponse {
        success: true,
        data: instruction_data,
    };

    (StatusCode::OK, Json(response))
}

async fn mint_token_handler(Json(payload): Json<MintTokenRequest>) -> impl IntoResponse {
    let accounts = vec![
        AccountMeta {
            pubkey: payload.mint.clone(),
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: payload.destination.clone(),
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: payload.authority.clone(),
            is_signer: true,
            is_writable: false,
        },
    ];

    let mut instruction_bytes = vec![7u8];
    instruction_bytes.extend_from_slice(&payload.amount.to_le_bytes());

    let instruction_data = InstructionData {
        program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
        accounts,
        instruction_data: base64::engine::general_purpose::STANDARD.encode(&instruction_bytes),
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
        .route("/token/create", post(create_token_handler))
        .route("/token/mint", post(mint_token_handler));
        // .route("/message/sign", post(sign_message_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    
    println!("Server is running on http://{}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}