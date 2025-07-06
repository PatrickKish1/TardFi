#!/bin/bash

# Oil Tokenization Platform Database Seeder
echo "🌱 Starting Oil Tokenization Platform Database Seeder..."

# Check if .env file exists
if [ ! -f .env ]; then
    echo "❌ Error: .env file not found!"
    echo "Please create a .env file with your DATABASE_URL"
    echo "Example: DATABASE_URL=postgres://user:password@localhost:5432/oil_tokenization"
    exit 1
fi

# Check if oil_tokens.json exists
if [ ! -f data/oil_tokens.json ]; then
    echo "❌ Error: data/oil_tokens.json not found!"
    echo "Please ensure the oil tokens data file exists"
    exit 1
fi

# Run the seeder
echo "🚀 Running database seeder..."
cargo run

if [ $? -eq 0 ]; then
    echo "✅ Database seeding completed successfully!"
    echo "🎉 Oil Tokenization Platform is ready!"
else
    echo "❌ Database seeding failed!"
    exit 1
fi 