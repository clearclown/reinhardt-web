//! Content negotiation for Reinhardt
//!
//! This crate provides DRF-style content negotiation for selecting
//! the appropriate renderer and parser based on Accept headers.
//!
//! ## Planned Features
//! TODO: カスタムネゴシエーション戦略 - プラグイン可能なネゴシエーションロジック
//! TODO: Content-Type検出 - リクエストボディのContent-Type自動検出
//! TODO: 言語ネゴシエーション - Accept-Language headerのサポート
//! TODO: エンコーディングネゴシエーション - Accept-Encoding headerのサポート
//! TODO: キャッシュ最適化 - ネゴシエーション結果のキャッシング
//! TODO: より詳細なエラー情報 - ネゴシエーション失敗時の詳細なフィードバック

pub mod accept;
pub mod media_type;
pub mod negotiator;

pub use media_type::MediaType;
pub use negotiator::{
    BaseContentNegotiation, BaseNegotiator, ContentNegotiator, NegotiationError, RendererInfo,
};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::media_type::*;
    pub use crate::negotiator::*;
}
