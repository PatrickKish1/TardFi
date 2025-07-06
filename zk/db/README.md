# Oil Tokenization Platform Database Seeder

This directory contains the database seeder for the Oil Tokenization Platform, which populates the database with sample oil token data.

## Overview

The seeder creates sample data for:

- **Oil Tokens**: Various types of oil assets (crude oil, refined products, natural gas liquids)
- **Tokenizations**: Digital tokenization records for oil assets
- **Comments**: User comments on oil tokens
- **Saved Tokens**: User saved token preferences

## Sample Data

The seeder includes 5 sample oil tokens:

1. **Premium Crude Oil (Brent)** - North Sea, UK
2. **WTI Crude Oil** - Cushing, Oklahoma, USA
3. **Ultra Low Sulfur Diesel** - Rotterdam, Netherlands
4. **Jet A-1 Fuel** - Dubai, UAE
5. **Natural Gas Liquids (Ethane)** - Permian Basin, Texas, USA

## Setup

### 1. Environment Configuration

Create a `.env` file in the `db` directory:

```env
DATABASE_URL=postgres://user:password@localhost:5432/oil_tokenization
```

### 2. Database Setup

Ensure your PostgreSQL database is running and the oil tokenization tables are created:

```bash
# Run migrations first
cd db
cargo run --bin migration

# Then run the seeder
cargo run
```

### 3. Quick Start

Use the provided script for easy setup:

```bash
cd db
./run_seeder.sh
```

## Data Structure

### Oil Token Fields

- `token_id`: Unique identifier for the oil token
- `oil_type`: Type of oil (Crude Oil, Refined Product, Natural Gas Liquids)
- `grade`: Specific grade (Brent, WTI, Ultra Low Sulfur Diesel, etc.)
- `quantity`: Amount of oil
- `unit`: Unit of measurement (barrels, gallons, cubic_meters)
- `location`: Geographic location
- `certification`: Quality certification standard
- `quality_report`: JSON object with quality specifications
- `storage_conditions`: JSON object with storage requirements
- `expiry_date`: Expiration date for the oil
- `current_price`: Current market price
- `reserve_price`: Reserve/minimum price
- `status`: Token status (active, pending, sold, expired, cancelled)
- `owner`: Ethereum address of the token owner

### Quality Reports

Each oil token includes detailed quality specifications:

- **Crude Oil**: API gravity, sulfur content, viscosity, color
- **Refined Products**: Cetane number, flash point, density, sulfur content
- **Natural Gas Liquids**: Composition analysis, vapor pressure

### Storage Conditions

Storage requirements include:

- Temperature ranges
- Pressure conditions
- Container types
- Facility locations

## Usage

### Manual Seeding

```bash
cd db
cargo run
```

### Programmatic Seeding

```rust
use db::seeder::seed_database;
use sea_orm::Database;

#[tokio::main]
async fn main() {
    let db = Database::connect("your_database_url").await.unwrap();
    seed_database(&db).await.unwrap();
}
```

## Customization

### Adding New Oil Tokens

1. Edit `data/oil_tokens.json` to add new oil token data
2. Follow the existing JSON structure
3. Run the seeder again

### Modifying Sample Data

- **Oil Tokens**: Edit `data/oil_tokens.json`
- **Tokenizations**: Modify `src/seeder.rs` in the `seed_tokenizations` function
- **Comments**: Modify `src/seeder.rs` in the `seed_comments` function
- **Saved Tokens**: Modify `src/seeder.rs` in the `seed_saved_tokens` function

## Troubleshooting

### Common Issues

1. **Database Connection Error**

   - Verify DATABASE_URL in .env file
   - Ensure PostgreSQL is running
   - Check database permissions

2. **Missing Data File**

   - Ensure `data/oil_tokens.json` exists
   - Verify JSON format is valid

3. **Migration Errors**
   - Run migrations before seeding
   - Check database schema matches entity definitions

### Logs

The seeder provides detailed logging:

- ✅ Success messages for each step
- ❌ Error messages with details
- 🌱 Progress indicators

## API Integration

After seeding, the data is available through the API endpoints:

- `GET /api/oil_tokens` - List all oil tokens
- `GET /api/tokenizations` - List all tokenizations
- `GET /api/comments` - List all comments
- `GET /api/saved_tokens/{user}` - List saved tokens by user

## Zero-Knowledge Proof Integration

The seeded data can be used to initialize zero-knowledge proof states:

- `GET /api/oil_tokens/init` - Initialize oil token ZK state
- `GET /api/tokenizations/init` - Initialize tokenization ZK state
- `GET /api/comments/init` - Initialize comment ZK state
- `GET /api/db/init` - Initialize overall database state
