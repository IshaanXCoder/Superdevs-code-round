use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use axum::{Router, Json, routing::{get, post}, http::StatusCode, response::IntoResponse};
use serde::{Serialize, Deserialize};
use base64::Engine;
use std::net::SocketAddr;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

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

#[derive(Deserialize)]
struct SendSolRequest {
    from: String,
    to: String,
    lamports: u64,
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

async fn sign_message_handler(Json(payload): Json<SignMessageRequest>) -> impl IntoResponse {
    if payload.message.is_empty() || payload.secret.is_empty() {
        return error_response("Missing required fields").into_response();
    }

    let secret_bytes = match bs58::decode(&payload.secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return error_response("Invalid secret key format").into_response(),
    };

    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(kp) => kp,
        Err(_) => return error_response("Invalid secret key").into_response(),
    };

    let message_bytes = payload.message.as_bytes();
    
    // Use try_sign_message for proper message signing
    let signature = match keypair.try_sign_message(message_bytes) {
        Ok(sig) => sig,
        Err(_) => return error_response("Failed to sign message").into_response(),
    };

    let response_data = SignatureData {
        signature: base64::engine::general_purpose::STANDARD.encode(signature.as_ref()),
        public_key: keypair.pubkey().to_string(),
        message: payload.message,
    };

    let response = ApiResponse {
        success: true,
        data: response_data,
    };

    (StatusCode::OK, Json(response)).into_response()
}

async fn send_sol_handler(Json(payload): Json<SendSolRequest>) -> impl IntoResponse {
    if payload.from.is_empty() || payload.to.is_empty() {
        return error_response("Missing required fields").into_response();
    }

    if payload.lamports == 0 {
        return error_response("Amount must be greater than 0").into_response();
    }

    let from_pubkey = match payload.from.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => return error_response("Invalid sender address").into_response(),
    };

    let to_pubkey = match payload.to.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => return error_response("Invalid recipient address").into_response(),
    };

    if from_pubkey == to_pubkey {
        return error_response("Cannot send SOL to the same address").into_response();
    }

    let accounts = vec![
        AccountMeta {
            pubkey: payload.from.clone(),
            is_signer: true,
            is_writable: true,
        },
        AccountMeta {
            pubkey: payload.to.clone(),
            is_signer: false,
            is_writable: true,
        },
    ];

    let mut instruction_bytes = vec![2u8, 0u8, 0u8, 0u8];
    instruction_bytes.extend_from_slice(&payload.lamports.to_le_bytes());

    let instruction_data = InstructionData {
        program_id: "11111111111111111111111111111112".to_string(),
        accounts,
        instruction_data: base64::engine::general_purpose::STANDARD.encode(&instruction_bytes),
    };

    let response = ApiResponse {
        success: true,
        data: instruction_data,
    };

    (StatusCode::OK, Json(response)).into_response()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/keypair", get(keypair_handler))
        .route("/token/create", post(create_token_handler))
        .route("/token/mint", post(mint_token_handler))
        .route("/message/sign", post(sign_message_handler))
        .route("/send/sol", post(send_sol_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    
    println!("Server is running on http://{}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}