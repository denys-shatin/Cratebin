# 📦 Cratebin

Self-hosted snippet sharing service with TTL, privacy controls, and secure access.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Docker](https://img.shields.io/badge/Docker-Ready-2496ED?logo=docker&logoColor=white)](docker-compose.yml)

**🌐 Live Demo:** [https://cratebin.biz](https://cratebin.biz)

## Features

- **Multiple Visibility Levels** - Public, unlisted, and private snippets
- **Time-To-Live (TTL)** - Auto-expiration: 1h, 24h, 7d, or never
- **Password Protection** - Optional password for private snippets
- **Raw Mode** - Plain text access without HTML
- **Secure** - Argon2 hashing, rate limiting, UUID v4 IDs
- **Auto Cleanup** - Background task removes expired snippets
- **Clean UI** - GitHub-inspired dark theme

## Quick Start

```bash
# Clone and start
git clone https://github.com/denys-shatin/Cratebin.git
cd Cratebin
docker-compose up -d

# Access
# Frontend: http://localhost:3000
# Backend:  http://localhost:8080
```

## Tech Stack

- **Backend**: Rust, Axum, SQLx, PostgreSQL, Tokio
- **Frontend**: SvelteKit, TypeScript
- **Testing**: 60+ tests including 20+ property-based tests (QuickCheck)

## API

### Create Snippet
```bash
curl -X POST http://localhost:8080/snippets \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Hello, World!",
    "visibility": "public",
    "ttl": "24h"
  }'
```

### Get Snippet
```bash
curl http://localhost:8080/snippets/{id}
```

### Get Raw
```bash
curl http://localhost:8080/snippets/{id}/raw
```

### Delete Snippet
```bash
curl -X DELETE http://localhost:8080/snippets/{id} \
  -H "Content-Type: application/json" \
  -d '{"delete_token": "your-token"}'
```

## Configuration

### Backend (.env)
```env
DATABASE_URL=postgresql://cratebin:cratebin@db:5432/cratebin
RATE_LIMIT=100
MAX_CONTENT_SIZE=524288
CLEANUP_INTERVAL=3600
CORS_ORIGINS=http://localhost:3000
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
```

### Frontend (.env)
```env
PUBLIC_API_URL=http://backend:8080
PUBLIC_APP_URL=http://localhost:3000
```

## Manual Setup

### Backend
```bash
cd backend
cp .env.example .env
cargo run
```

### Frontend
```bash
cd frontend
cp .env.example .env
npm install
npm run dev
```

### Database
```bash
docker run -d \
  --name cratebin-db \
  -e POSTGRES_DB=cratebin \
  -e POSTGRES_USER=cratebin \
  -e POSTGRES_PASSWORD=cratebin \
  -p 5432:5432 \
  postgres:15-alpine
```

## Development

### Run Tests
```bash
# Backend
cd backend && cargo test

# Frontend
cd frontend && npm test
```

### Project Structure
```
Cratebin/
├── backend/          # Rust/Axum API
│   ├── src/         # Source code
│   └── migrations/  # SQL migrations
├── frontend/        # SvelteKit app
│   └── src/         # Source code
└── docker-compose.yml
```

## Security

- Argon2id password hashing
- Rate limiting (100 req/min per IP)
- UUID v4 non-sequential IDs
- Input validation (512KB max, UTF-8 only)
- CORS protection
- Constant-time password comparison

## Contributing

Pull requests welcome! For major changes, open an issue first.

```bash
# Fork, clone, create branch
git checkout -b feature/amazing-feature

# Make changes, test
cargo test
npm test

# Commit and push
git commit -m 'Add amazing feature'
git push origin feature/amazing-feature
```

## License

MIT License - see [LICENSE](LICENSE)

## Author

**Denys Shatin** - [@denys-shatin](https://github.com/denys-shatin)

---

Built with Rust, Axum, SvelteKit, and PostgreSQL
