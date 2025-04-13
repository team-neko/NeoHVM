use axum::{Json, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, EncodingKey, Header};
use rusqlite::{Connection, Result as SqliteResult};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use windows::Win32::Security::LogonUserW;
use windows::Win32::Security::LOGON32_LOGON_NETWORK;
use windows::Win32::Security::LOGON32_PROVIDER_DEFAULT;
use windows::core::PWSTR;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
    status: String,
}

pub async fn login_handler(Json(login): Json<LoginRequest>) -> impl IntoResponse {
    match authenticate_windows(&login.username, &login.password) {
        Ok(_) => {
            // 비밀번호를 해시화하여 시크릿 키로 사용
            let mut hasher = Sha256::new();
            hasher.update(login.password.as_bytes());
            let secret_key = hasher.finalize();

            let expiration = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize + 24 * 60 * 60; // 24시간

            let claims = Claims {
                sub: login.username.clone(),
                exp: expiration,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(&secret_key)
            ).unwrap();

            // SQLite에 토큰 저장
            if let Err(e) = store_token(&login.username, &token) {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(LoginResponse {
                        token: "".to_string(),
                        status: format!("데이터베이스 오류: {}", e)
                    })
                );
            }

            (
                StatusCode::OK,
                Json(LoginResponse {
                    token,
                    status: "success".to_string()
                })
            )
        },
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            Json(LoginResponse {
                token: "".to_string(),
                status: format!("인증 실패: {}", e)
            })
        )
    }
}

fn authenticate_windows(username: &str, password: &str) -> Result<(), String> {
    let domain = ""; // 도메인이 없는 경우 빈 문자열
    
    let username_wide: Vec<u16> = OsStr::new(username)
        .encode_wide()
        .chain(Some(0))
        .collect();
    let password_wide: Vec<u16> = OsStr::new(password)
        .encode_wide()
        .chain(Some(0))
        .collect();
    let domain_wide: Vec<u16> = OsStr::new(domain)
        .encode_wide()
        .chain(Some(0))
        .collect();

    let mut handle = HANDLE::default();

    unsafe {
        let result = LogonUserW(
            PWSTR(username_wide.as_ptr() as *mut _),
            PWSTR(domain_wide.as_ptr() as *mut _),
            PWSTR(password_wide.as_ptr() as *mut _),
            LOGON32_LOGON_NETWORK,
            LOGON32_PROVIDER_DEFAULT,
            &mut handle,
        );

        if result.is_ok() {
            // 핸들을 정리합니다
            if !handle.is_invalid() {
                let _ = CloseHandle(handle);
            }
            Ok(())
        } else {
            if !handle.is_invalid() {
                let _ = CloseHandle(handle);
            }
            Err("Windows 인증 실패".to_string())
        }
    }
}

fn store_token(username: &str, token: &str) -> SqliteResult<()> {
    // data 디렉토리가 없으면 생성
    fs::create_dir_all("data").unwrap_or_else(|_| {
        println!("data 디렉토리를 생성할 수 없습니다.");
    });
    
    let conn = Connection::open("data/auth.db")?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tokens (
            username TEXT PRIMARY KEY,
            token TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "INSERT OR REPLACE INTO tokens (username, token) VALUES (?1, ?2)",
        [username, token],
    )?;

    Ok(())
}
