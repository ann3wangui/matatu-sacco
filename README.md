# SACCO Management System

## Overview
The SACCO Management System is a Rust-based decentralized application (dApp) designed to manage the operations of a SACCO (Savings and Credit Cooperative Organization) and its associated assets, such as matatus (public transport vehicles), drivers, and trips. The system provides functionality for handling SACCO details, vehicle registration, driver assignment, trip management, revenue and expense tracking, and real-time analytics. 

The system is implemented using the Internet Computer (IC) framework, leveraging stable data structures for persistence and candid serialization for interoperability.

## Features
### Core Functionality:
- **SACCO Management**: Create and manage SACCOs with detailed contact and location information.
- **Matatu Registration**: Register matatus with capacity, route, and status information.
- **Driver Management**: Register drivers, assign them to matatus, and track their performance.
- **Trip Management**: Start, end, and manage trips, including passenger counts and revenue.
- **Revenue and Expense Tracking**: Record and analyze revenues and expenses with detailed breakdowns.
- **Route Optimization**: Optimize travel routes based on traffic patterns and historical data.
- **Real-Time Tracking**: Update and track matatu locations in real-time.
- **Financial Reporting**: Generate comprehensive financial reports for SACCOs.

### Analytics and Feedback:
- **Driver Performance Analytics**: Monitor driver performance based on trip completion, revenue generation, and customer feedback.
- **Matatu Analytics**: View total trips, revenue, maintenance costs, fuel costs, and net profit for each matatu.
- **Customer Feedback**: Collect and analyze customer feedback on trips.

## Architecture
The application uses the following key components:
- **Rust**: The primary programming language for building the application.
- **Internet Computer (IC)**: Enables decentralized computation and storage.
- **Candid**: A serialization format used for interoperability with other canisters.
- **Stable Structures**: Ensures long-term data persistence on the IC platform.

### Key Data Structures
1. **SACCO**: Contains details about the SACCO.
2. **Matatu**: Represents a vehicle managed by a SACCO.
3. **Driver**: Stores driver details and their assigned matatu.
4. **Trip**: Tracks trip data, including revenue and passengers.
5. **Revenue**: Logs revenue details for SACCO operations.
6. **Expense**: Logs expense details for SACCO operations.
7. **Route**: Stores route details and optimization data.
8. **CustomerFeedback**: Collects and stores customer feedback.

### Memory Management
The project uses `StableBTreeMap` to ensure data persistence across canister upgrades. Data is stored in key-value pairs where keys are unique IDs.

## Installation

### Prerequisites
- Rust (with `cargo` and `rustc` installed)
- Internet Computer SDK

### Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/ann3wangui/matatu-sacco.git
   cd matatu-sacco
   ```

## Usage

### Deploy the System
1. Start the local replica:
   ```bash
   dfx start --background
   ```
2. Deploy the application:
   ```bash
   npm run gen-deploy
   ```

### Interact with the System
The system exposes the following endpoints:
- `create_sacco`: Create a new SACCO.
- `register_matatu`: Register a new matatu.
- `register_driver`: Register a new driver.
- `start_trip`: Start a new trip.
- `end_trip`: End an ongoing trip.
- `generate_financial_report`: Generate a financial report for a given period.
- `optimize_route`: Optimize a route based on current traffic conditions.

### Example
Use the `dfx canister call` command to interact with the deployed canister:
```bash
# Create a SACCO
dfx canister call sacco_management create_sacco '("My SACCO", "Nairobi", "123456789", "email@example.com")'

# Register a Matatu
dfx canister call sacco_management register_matatu '(1, "KAB123C", 14, "Route A")'
```

## Contributing
Contributions are welcome! To contribute:
1. Fork the repository.
2. Create a new branch:
   ```bash
   git checkout -b feature-name
   ```
3. Commit your changes and push to the branch:
   ```bash
   git commit -m "Description of changes"
   git push origin feature-name
   ```
4. Submit a pull request.

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.

## Acknowledgments
- [Internet Computer](https://internetcomputer.org/) for providing the framework.
- [Serde](https://serde.rs/) for serialization and deserialization.
- [Rust](https://www.rust-lang.org/) for the language and ecosystem.
