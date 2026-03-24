# 🥪 Jaffle Shop Generator 🏭

The Jaffle Shop Generator (`jafgen`) is a data generator that creates synthetic datasets suitable for analytics engineering practice or demonstrations. It provides a **web UI** at `localhost:3000` where you can configure and generate data in CSV format, designed to be used with a relational database.

The generated data includes tables for:

- **Customers** (who place Orders)
- **Orders** (from those Customers)
- **Products** (the food and beverages the Orders contain)
- **Order Items** (of those Products)
- **Supplies** (needed for making those Products)
- **Stores** (where the Orders are placed and fulfilled)
- **Tweets** (Customers sometimes issue Tweets after placing an Order)

## Installation

_Requires [Rust](https://www.rust-lang.org/tools/install). The install script will set it up for you if you don't have it._

```shell
git clone https://github.com/matthewshaver/jaffle-shop-generator-2.git
cd jaffle-shop-generator-2
./install.sh
```

This builds and installs the `jafgen` binary to `~/.cargo/bin/`. If this is your first time installing Rust, restart your terminal (or run `source "$HOME/.cargo/env"`) so `jafgen` is on your PATH.

## Usage

From any directory, run:

```shell
jafgen
```

This will:
1. Start a local web server
2. Automatically open `http://localhost:3000` in your browser
3. Present the configuration UI

### Web UI

The web interface lets you configure:

- **Start Date / End Date** — the time range for the simulation
- **Max Orders** — optionally cap the total number of orders generated
- **File Prefix** — prefix for output CSV filenames (default: `raw`)
- **Store Locations** — add, remove, and configure stores with:
  - Name, city, and country
  - Popularity, tax rate, and market size
  - Opening day offset from the start date

Click **Generate Data** and the CSVs will be written to a `jaffle-data/` folder in your current working directory.

### International Stores

Stores can be assigned to any country. When stores span multiple countries, the stores CSV files are split by country with a suffix (e.g. `raw_stores_US.csv`, `raw_stores_GB.csv`).

### Default Stores

Six US locations are pre-loaded by default:

| Store | Popularity | Tax Rate | Market Size |
|-------|-----------|----------|-------------|
| Philadelphia | 0.85 | 6% | 900 |
| Brooklyn | 0.95 | 4% | 1,400 |
| Chicago | 0.92 | 6.25% | 1,200 |
| San Francisco | 0.87 | 7.5% | 1,100 |
| New Orleans | 0.92 | 4% | 800 |
| Los Angeles | 0.87 | 8% | 800 |

## Output

Generated CSV files are saved to `./jaffle-data/` in your current directory:

| File | Description |
|------|-------------|
| `{prefix}_customers.csv` | Customer IDs and names |
| `{prefix}_orders.csv` | Orders with timestamps, store, subtotal, tax, and total (in cents) |
| `{prefix}_items.csv` | Order-to-SKU line items |
| `{prefix}_products.csv` | Product catalog (SKU, name, type, price, description) |
| `{prefix}_supplies.csv` | Supplies with cost, perishability, and linked SKUs |
| `{prefix}_stores.csv` | Store details (name, open date, tax rate) |
| `{prefix}_tweets.csv` | Customer tweets with sentiment |

## How It Works

Rather than using discrete rules to generate data, `jafgen` sets up entities with behavior patterns and lets them interact over simulated time. Customers have personas (Commuter, Remote Worker, Brunch Crowd, Student, Casuals, Health Nut) that determine when and what they buy. The simulation applies:

- **Seasonality** — a cosine curve across the year
- **Weekend effects** — reduced traffic on Saturdays and Sundays
- **Growth trends** — ~20% year-over-year increase
- **Market penetration** — new stores gradually ramp up their customer base

Each run produces unique data. The simulation is not idempotent by design.

## Development

```shell
git clone https://github.com/dbt-labs/jaffle-shop-generator.git
cd jaffle-shop-generator
cargo build --release
cargo run --release
```

The project structure:

```
Cargo.toml          # Rust package manifest
src/
  main.rs           # Web server entry point (axum, port 3000)
  models/           # Data models (items, stores, customers, orders, tweets, supplies)
  simulation/       # Time, curves, market penetration, simulation orchestrator
  web/              # HTTP handlers and API endpoints
static/
  index.html        # Web UI (embedded in binary at compile time)
install.sh          # One-line install script
```

### API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | Web UI |
| GET | `/api/defaults` | Default configuration (dates + stores) |
| POST | `/api/generate` | Run simulation and write CSVs |
