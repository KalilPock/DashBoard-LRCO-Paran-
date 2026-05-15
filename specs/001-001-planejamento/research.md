# Research: DashBoard de Escolas e Sincronia LRCO

## Technical Decisions

### 1. Language & Ecosystem
- **Decision**: Rust
- **Rationale**: Project already in Rust; excellent performance and safety guarantees for data synchronization.
- **Alternatives considered**: TypeScript/Node.js (for web ecosystem, but decided against given Rust focus).

### 2. Backend API & UI Framework
- **Decision**: Axum (Backend) + Leptos or Yew (Frontend)
- **Rationale**: Leptos/Yew provide powerful Rust-based web UI capabilities; Axum is idiomatic for modern Rust web services.
- **Alternatives considered**: Actix-web (backend), standard REST SPA (TypeScript).

### 3. Local Storage/Database
- **Decision**: SQLite with `sqlx`
- **Rationale**: Lightweight, robust, excellent Rust support, perfect for local storage requirement and offline support.
- **Alternatives considered**: JSON file-based storage (insufficient for complex relations/data integrity), IndexedDB (via WASM/Leptos).

## Research Tasks Resolution
- **Rust Version**: Using stable Rust 1.75+ (aligned with modern async ecosystem).
- **Primary Dependencies**: `axum`, `leptos` (or `yew`), `sqlx`, `reqwest` (for LRCO API client).
- **Storage**: SQLite via `sqlx`.
