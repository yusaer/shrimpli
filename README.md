# 🦐 Shrimpli

A URL shortening service built with Rust.
*"Small and simple, like a shrimp."*

## Tech Stack

### Backend
- **Rust** with Axum framework
- **SQLx** for database operations
- **PostgreSQL** for data persistence

### Frontend
- **React 19** with TypeScript
- **Vite** for build tooling

### Infrastructure (Planned)
- **AWS App Runner** (Phase 1)
- **AWS ECS Fargate** + ALB (Phase 2)
- **GitHub Actions** for CI/CD
- **Docker** for containerization

## Features

- Shorten long URLs into 6-character short codes
- Redirect to original URLs via `GET /{short_code}`
- Track access counts for each shortened URL

## Local Development

### Prerequisites

- Rust (latest stable)
- Node.js (v20+)
- Docker & Docker Compose

### Setup

1. **Clone the repository**

```bash
git clone https://github.com/yusaer/shrimpli.git
cd shrimpli
```

2. **Start PostgreSQL**

```bash
docker compose up -d
```

3. **Setup Backend**

```bash
cd backend

# Run database migrations
sqlx migrate run

# Start the API server
cargo run
```

The API will be available at `http://localhost:8080`

4. **Setup Frontend**

```bash
cd frontend
npm install
npm run dev
```

The frontend will be available at `http://localhost:5173`

## API Usage

### Shorten a URL

```bash
curl -X POST http://localhost:8080/urls \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com/very/long/path"}'
```

Response:
```json
{
  "short_code": "abc123",
  "short_url": "http://localhost:8080/abc123",
  "original_url": "https://example.com/very/long/path"
}
```

### Access a shortened URL

```bash
curl -I http://localhost:8080/abc123
```

This returns a `302 Found` redirect to the original URL.

### Get URL statistics

```bash
curl http://localhost:8080/urls/abc123/stats
```

Response:
```json
{
  "short_code": "abc123",
  "original_url": "https://example.com/very/long/path",
  "access_count": 42,
  "created_at": "2025-01-30T12:00:00Z"
}
```

## Project Structure

```
shrimpli/
├── backend/           # Rust API (shrimpli-api)
│   ├── src/
│   ├── migrations/
│   └── Cargo.toml
├── frontend/          # React app (shrimpli-web)
│   ├── src/
│   └── package.json
├── infra/             # Terraform configurations
├── .github/workflows/ # CI/CD pipelines
└── docker-compose.yml
```

## Roadmap

- [x] Backend API implementation
- [x] Frontend UI
- [ ] Terraform infrastructure setup
- [ ] GitHub Actions CI/CD pipeline
- [ ] Deploy to AWS App Runner (Phase 1)
- [ ] Migrate to ECS Fargate (Phase 2)

## License

MIT
