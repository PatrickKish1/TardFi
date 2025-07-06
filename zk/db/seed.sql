-- Database Seed File for Oil Tokenization Platform

-- Clear existing data (if any)
TRUNCATE TABLE saved_token CASCADE;
TRUNCATE TABLE comment CASCADE;
TRUNCATE TABLE tokenization CASCADE;
TRUNCATE TABLE oil_token CASCADE;

-- Reset sequences
ALTER SEQUENCE oil_token_id_seq RESTART WITH 1;
ALTER SEQUENCE tokenization_id_seq RESTART WITH 1;
ALTER SEQUENCE comment_id_seq RESTART WITH 1;
ALTER SEQUENCE saved_token_id_seq RESTART WITH 1;

-- Insert Oil Tokens
INSERT INTO oil_token (
    id,
    token_id,
    oil_type,
    grade,
    quantity,
    unit,
    location,
    certification,
    quality_report,
    storage_conditions,
    expiry_date,
    current_price,
    reserve_price,
    status,
    owner,
    created_at,
    updated_at
) VALUES
    -- Premium Crude Oil Token (ID: 1)
    (
        1,
        1001,
        'Crude Oil',
        'Brent',
        1000.00,
        'barrels',
        'North Sea, UK',
        'ISO 9001:2015',
        '{"api_gravity": 38.5, "sulfur_content": 0.37, "viscosity": "medium", "color": "light_amber", "tests": ["flash_point", "pour_point", "cloud_point"]}',
        '{"temperature": "15-25°C", "pressure": "atmospheric", "container": "steel_tank", "location": "onshore_facility"}',
        '2025-12-31T23:59:59Z',
        75000.00,
        70000.00,
        'active',
        '0x1234567890abcdef1234567890abcdef12345678',
        '2024-01-01T00:00:00Z',
        '2024-01-01T00:00:00Z'
    ),

    -- WTI Crude Oil Token (ID: 2)
    (
        2,
        1002,
        'Crude Oil',
        'WTI',
        500.00,
        'barrels',
        'Cushing, Oklahoma, USA',
        'API MPMS Chapter 8.1',
        '{"api_gravity": 39.6, "sulfur_content": 0.24, "viscosity": "low", "color": "golden", "tests": ["flash_point", "pour_point", "cloud_point", "distillation"]}',
        '{"temperature": "20-30°C", "pressure": "atmospheric", "container": "pipeline", "location": "hub_facility"}',
        '2025-06-30T23:59:59Z',
        38000.00,
        35000.00,
        'active',
        '0x2345678901bcdef2345678901bcdef2345678901',
        '2024-01-02T00:00:00Z',
        '2024-01-02T00:00:00Z'
    ),

    -- Refined Diesel Token (ID: 3)
    (
        3,
        1003,
        'Refined Product',
        'Ultra Low Sulfur Diesel',
        2000.00,
        'gallons',
        'Rotterdam, Netherlands',
        'EN 590:2013',
        '{"cetane_number": 51, "sulfur_content": 0.001, "density": "0.832 kg/l", "flash_point": "55°C", "tests": ["cetane_index", "cold_filter_plugging_point"]}',
        '{"temperature": "10-40°C", "pressure": "atmospheric", "container": "tank_farm", "location": "port_facility"}',
        '2025-03-31T23:59:59Z',
        8500.00,
        8000.00,
        'active',
        '0x3456789012cdef3456789012cdef3456789012c',
        '2024-01-03T00:00:00Z',
        '2024-01-03T00:00:00Z'
    ),

    -- Jet Fuel Token (ID: 4)
    (
        4,
        1004,
        'Refined Product',
        'Jet A-1',
        1500.00,
        'gallons',
        'Dubai, UAE',
        'ASTM D1655',
        '{"flash_point": "38°C", "freezing_point": "-47°C", "density": "0.804 kg/l", "sulfur_content": 0.003, "tests": ["thermal_stability", "electrical_conductivity"]}',
        '{"temperature": "15-25°C", "pressure": "atmospheric", "container": "aviation_fuel_tank", "location": "airport_facility"}',
        '2025-09-30T23:59:59Z',
        12000.00,
        11000.00,
        'active',
        '0x4567890123def4567890123def4567890123def',
        '2024-01-04T00:00:00Z',
        '2024-01-04T00:00:00Z'
    ),

    -- Natural Gas Liquids Token (ID: 5)
    (
        5,
        1005,
        'Natural Gas Liquids',
        'Ethane',
        5000.00,
        'cubic_meters',
        'Permian Basin, Texas, USA',
        'ASTM D1835',
        '{"ethane_content": 95.5, "propane_content": 3.2, "butane_content": 1.3, "impurities": "trace", "tests": ["composition_analysis", "vapor_pressure"]}',
        '{"temperature": "-88°C", "pressure": "high_pressure", "container": "cryogenic_tank", "location": "gas_plant"}',
        '2025-12-31T23:59:59Z',
        25000.00,
        22000.00,
        'active',
        '0x5678901234ef5678901234ef5678901234ef567',
        '2024-01-05T00:00:00Z',
        '2024-01-05T00:00:00Z'
    );

-- Insert Tokenization Records
INSERT INTO tokenization (
    id,
    oil_token_id,
    tokenizer_id,
    amount,
    created_at,
    updated_at
) VALUES
    (1, 1, '0x1234567890abcdef1234567890abcdef12345678', 75000.00, '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z'),
    (2, 2, '0x2345678901bcdef2345678901bcdef2345678901', 38000.00, '2024-01-02T00:00:00Z', '2024-01-02T00:00:00Z'),
    (3, 3, '0x3456789012cdef3456789012cdef3456789012c', 8500.00, '2024-01-03T00:00:00Z', '2024-01-03T00:00:00Z'),
    (4, 4, '0x4567890123def4567890123def4567890123def', 12000.00, '2024-01-04T00:00:00Z', '2024-01-04T00:00:00Z'),
    (5, 5, '0x5678901234ef5678901234ef5678901234ef567', 25000.00, '2024-01-05T00:00:00Z', '2024-01-05T00:00:00Z');

-- Insert Comments
INSERT INTO comment (
    id,
    oil_token_id,
    user,
    content,
    created_at,
    updated_at
) VALUES
    (1, 1, '0x1234567890abcdef1234567890abcdef12345678', 'High quality Brent crude with excellent specifications for refining.', '2024-01-01T10:00:00Z', '2024-01-01T10:00:00Z'),
    (2, 2, '0x2345678901bcdef2345678901bcdef2345678901', 'WTI crude from Cushing hub - premium grade for US markets.', '2024-01-02T11:00:00Z', '2024-01-02T11:00:00Z'),
    (3, 3, '0x3456789012cdef3456789012cdef3456789012c', 'ULSD meets all European specifications for road transport.', '2024-01-03T12:00:00Z', '2024-01-03T12:00:00Z'),
    (4, 4, '0x4567890123def4567890123def4567890123def', 'Jet A-1 fuel certified for international aviation use.', '2024-01-04T13:00:00Z', '2024-01-04T13:00:00Z'),
    (5, 5, '0x5678901234ef5678901234ef5678901234ef567', 'High-purity ethane from Permian Basin for petrochemical use.', '2024-01-05T14:00:00Z', '2024-01-05T14:00:00Z');

-- Insert Saved Tokens
INSERT INTO saved_token (
    id,
    user,
    oil_token_id,
    created_at
) VALUES
    (1, '0x1234567890abcdef1234567890abcdef12345678', 1, '2024-01-01T15:00:00Z'),
    (2, '0x2345678901bcdef2345678901bcdef2345678901', 2, '2024-01-02T16:00:00Z'),
    (3, '0x3456789012cdef3456789012cdef3456789012c', 3, '2024-01-03T17:00:00Z'),
    (4, '0x4567890123def4567890123def4567890123def', 4, '2024-01-04T18:00:00Z'),
    (5, '0x5678901234ef5678901234ef5678901234ef567', 5, '2024-01-05T19:00:00Z');