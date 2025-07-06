# Oil Tokenization Platform

This project has been transformed from a car auction platform into an oil tokenization platform with blockchain verification using zero-knowledge proofs.

## Overview

The Oil Tokenization Platform is a centralized database system that serves as a verification layer for oil tokenization on the blockchain. It uses zero-knowledge proofs to ensure data integrity and transparency while maintaining privacy.

## Key Features

### 1. Oil Token Management

- **Oil Token Creation**: Create digital representations of oil assets
- **Token Types**: Support for various oil types (Crude Oil, Refined Products, Natural Gas Liquids)
- **Quality Reports**: Detailed quality specifications and certifications
- **Storage Conditions**: Environmental and storage requirements tracking

### 2. Tokenization Process

- **Digital Tokenization**: Convert physical oil assets into digital tokens
- **Price Management**: Current and reserve price tracking
- **Ownership Transfer**: Secure ownership management with blockchain addresses
- **Status Tracking**: Token lifecycle management (pending, active, sold, expired, cancelled)

### 3. Zero-Knowledge Proof Integration

- **Merkle Tree Verification**: Cryptographic verification of database state
- **State Synchronization**: Real-time sync between database and blockchain
- **Proof Generation**: Generate ZK proofs for data integrity
- **Verification**: Verify proofs without revealing underlying data

### 4. API Endpoints

#### Oil Tokens

- `GET /api/oil_tokens` - Get all oil tokens
- `GET /api/oil_tokens/{id}` - Get specific oil token
- `POST /api/oil_tokens` - Create new oil token (protected)
- `GET /api/oil_tokens/init` - Initialize oil token ZK state

#### Tokenization

- `GET /api/tokenizations` - Get all tokenizations
- `GET /api/tokenizations/{id}` - Get specific tokenization
- `POST /api/tokenizations` - Create new tokenization (protected)
- `POST /api/tokenizations/{id}/complete` - Complete tokenization (protected)
- `GET /api/tokenizations/init` - Initialize tokenization ZK state

#### Comments

- `GET /api/comments` - Get all comments
- `GET /api/comments/{id}` - Get comments by oil token ID
- `POST /api/comments` - Create new comment (protected)
- `GET /api/comments/init` - Initialize comment ZK state

#### Saved Tokens

- `GET /api/saved_tokens/{user}` - Get saved tokens by user
- `GET /api/tokens/saved/{id}` - Get saved tokens by oil token ID
- `POST /api/save_token` - Save token (protected)

#### Authentication & State

- `GET /api/auth` - Get verification handler
- `POST /api/auth` - Verify signature
- `POST /api/auth/verify` - Verify authentication
- `GET /api/state` - Get overall state
- `GET /api/sync` - Sync state with blockchain
- `GET /api/db/init` - Initialize overall database state

## Database Schema

### Oil Token Table

```sql
CREATE TABLE oil_token (
    id SERIAL PRIMARY KEY,
    token_id INTEGER,
    oil_type VARCHAR,
    grade VARCHAR,
    quantity DECIMAL,
    unit VARCHAR,
    location VARCHAR,
    certification VARCHAR,
    quality_report JSON,
    storage_conditions JSON,
    expiry_date TIMESTAMP,
    current_price DECIMAL,
    reserve_price DECIMAL,
    status VARCHAR,
    owner VARCHAR,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);
```

### Tokenization Table

```sql
CREATE TABLE tokenization (
    id SERIAL PRIMARY KEY,
    oil_token_id INTEGER,
    tokenizer_id VARCHAR,
    amount DECIMAL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);
```

### Comment Table

```sql
CREATE TABLE comment (
    id SERIAL PRIMARY KEY,
    oil_token_id INTEGER,
    user VARCHAR,
    content TEXT,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);
```

### Saved Token Table

```sql
CREATE TABLE saved_token (
    id SERIAL PRIMARY KEY,
    user VARCHAR,
    oil_token_id INTEGER,
    created_at TIMESTAMP
);
```

## Sample Data

The platform includes sample oil token data:

1. **Premium Crude Oil Token** - Brent crude from North Sea
2. **WTI Crude Oil Token** - West Texas Intermediate from Cushing
3. **Refined Diesel Token** - Ultra Low Sulfur Diesel from Rotterdam
4. **Jet Fuel Token** - Jet A-1 from Dubai
5. **Natural Gas Liquids Token** - Ethane from Permian Basin

## Zero-Knowledge Proof Architecture

### State Management

- **Oil Token State**: Manages oil token data integrity
- **Tokenization State**: Handles tokenization process verification
- **Comment State**: Ensures comment authenticity
- **Overall State**: Combines all states for system-wide verification

### Proof Generation

1. **Data Leaves**: Convert database records to merkle tree leaves
2. **State Initialization**: Create initial ZK state from leaves
3. **Proof Generation**: Generate cryptographic proofs
4. **Verification**: Verify proofs on blockchain

### Blockchain Integration

- **IPFS Storage**: Store proofs on decentralized storage
- **Smart Contract Verification**: Verify proofs on blockchain
- **State Synchronization**: Keep database and blockchain in sync

## Technology Stack

- **Backend**: Rust with Axum framework
- **Database**: PostgreSQL with SeaORM
- **Zero-Knowledge**: RISC Zero zkVM
- **Blockchain**: Ethereum integration
- **Storage**: IPFS via Pinata
- **Authentication**: JWT with Ethereum signatures

## Getting Started

1. **Setup Database**:

   ```bash
   cd db
   cargo run --bin migration
   psql -d oil_tokenization -f seed.sql
   ```

2. **Build Methods**:

   ```bash
   cd methods
   cargo build
   ```

3. **Run Server**:

   ```bash
   cd host
   cargo run
   ```

4. **Environment Variables**:
   ```env
   DATABASE_URL=postgres://user:password@localhost:5432/oil_tokenization
   PINATA_API_KEY=your_pinata_api_key
   PINATA_SECRET_KEY=your_pinata_secret_key
   ```

## API Examples

### Create Oil Token

```bash
curl -X POST http://localhost:3001/api/oil_tokens \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <jwt_token>" \
  -d '{
    "token_id": 1001,
    "oil_type": "Crude Oil",
    "grade": "Brent",
    "quantity": 1000.00,
    "unit": "barrels",
    "location": "North Sea, UK",
    "certification": "ISO 9001:2015",
    "quality_report": {"api_gravity": 38.5, "sulfur_content": 0.37},
    "storage_conditions": {"temperature": "15-25°C", "pressure": "atmospheric"},
    "expiry_date": "2025-12-31T23:59:59Z",
    "current_price": 75000.00,
    "reserve_price": 70000.00,
    "status": "active"
  }'
```

### Get All Oil Tokens

```bash
curl http://localhost:3001/api/oil_tokens
```

### Initialize ZK State

```bash
curl http://localhost:3001/api/oil_tokens/init
```

## Security Features

- **JWT Authentication**: Secure API access
- **Ethereum Signature Verification**: Cryptographic identity verification
- **Zero-Knowledge Proofs**: Data integrity without revealing data
- **Merkle Tree Verification**: Tamper-evident data structures
- **Blockchain Integration**: Immutable audit trail

## Future Enhancements

1. **Smart Contract Integration**: Direct blockchain tokenization
2. **Real-time Price Feeds**: Integration with oil price APIs
3. **Advanced Analytics**: Token performance metrics
4. **Multi-chain Support**: Support for multiple blockchains
5. **Regulatory Compliance**: Built-in compliance features
6. **Mobile App**: Native mobile application
7. **API Rate Limiting**: Enhanced API security
8. **WebSocket Support**: Real-time updates

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
