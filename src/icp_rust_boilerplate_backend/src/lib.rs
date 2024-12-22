#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;
use std::collections::HashMap;

// SACCO struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct SACCO {
    id: u64,
    name: String,
    location: String,
    contact: String,
    email: String,
    created_at: u64,
}

// Matatu struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Matatu {
    id: u64,
    sacco_id: u64,
    plate_number: String,
    capacity: u32,
    route: String,
    status: String, // "active", "inactive", "maintenance"
}

// Matatu Analytics struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct MatatuAnalytics {
    total_trips: usize,
    total_revenue: f64,
    maintenance_costs: f64,
    fuel_costs: f64,
    net_profit: f64,
}

// Driver struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Driver {
    id: u64,
    sacco_id: u64,
    name: String,
    license_number: String,
    contact: String,
    assigned_matatu: Option<u64>, // Matatu ID
}

// Trip struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Trip {
    id: u64,
    matatu_id: u64,
    driver_id: u64,
    start_time: u64,
    end_time: Option<u64>,
    passengers: u32,
    route: String,
    status: String, // "ongoing", "completed", "cancelled"
    revenue: f64,
}

// Maintenance struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Maintenance {
    id: u64,
    matatu_id: u64,
    date: u64,
    description: String,
    cost: f64,
    status: String, // "scheduled", "in_progress", "completed"
}

// Driver Performance struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct DriverPerformance {
    id: u64,
    driver_id: u64,
    month: u64,
    trips_completed: u32,
    total_revenue: f64,
    customer_rating: f32,
    compliance_score: f32,
}

// Expense struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Expense {
    id: u64,
    sacco_id: u64,
    date: u64,
    category: String,
    amount: f64,
    description: String,
}

// Revenue struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Revenue {
    id: u64,
    sacco_id: u64,
    date: u64,
    matatu_id: u64,
    amount: f64,
    description: String,
}

// Fuel Consumption struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct FuelConsumption {
    id: u64,
    matatu_id: u64,
    date: u64,
    liters: f64,
    cost: f64,
    odometer_reading: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Route {
    id: u64,
    name: String,
    start_point: String,
    end_point: String,
    distance: f64,
    estimated_time: u32, // in minutes
    peak_hours: Vec<TimeWindow>,
    traffic_patterns: Vec<TrafficPattern>,
    average_passengers: u32,
    price: f64,
}

// Route Optimization struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RouteOptimization {
    route_id: u64,
    optimal_start_time: u64,
    estimated_duration: u32,
    congestion_level: u8,
    alternate_routes: Vec<Route>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct TimeWindow {
    start_hour: u8,
    end_hour: u8,
    day_of_week: u8, // 0-6 representing Sunday-Saturday
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct TrafficPattern {
    time_window: TimeWindow,
    congestion_level: u8, // 1-5 scale
    average_delay: u32,   // in minutes
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CustomerFeedback {
    id: u64,
    trip_id: u64,
    rating: u8,      // 1-5 scale
    cleanliness: u8, // 1-5 scale
    punctuality: u8, // 1-5 scale
    safety: u8,      // 1-5 scale
    comment: String,
    timestamp: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Schedule {
    id: u64,
    matatu_id: u64,
    driver_id: u64,
    route_id: u64,
    start_time: u64,
    end_time: u64,
    status: String, // "scheduled", "in_progress", "completed", "cancelled"
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct LocationUpdate {
    id: u64,
    matatu_id: u64,
    latitude: f64,
    longitude: f64,
    speed: f64,
    timestamp: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct FinancialReport {
    id: u64,
    sacco_id: u64,
    period_start: u64,
    period_end: u64,
    total_revenue: f64,
    total_expenses: f64,
    expense_breakdown: Vec<ExpenseCategory>,
    revenue_breakdown: Vec<RevenueSource>,
    profit_margin: f64,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct ExpenseCategory {
    category: String,
    amount: f64,
    percentage: f64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RevenueSource {
    source: String,
    amount: f64,
    percentage: f64,
}

// Payload structs
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateSACCOPayload {
    name: String,
    location: String,
    contact: String,
    email: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RegisterMatatuPayload {
    sacco_id: u64,
    plate_number: String,
    capacity: u32,
    route: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RegisterDriverPayload {
    sacco_id: u64,
    name: String,
    license_number: String,
    contact: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RecordExpensePayload {
    sacco_id: u64,
    category: String,
    amount: f64,
    description: String,
}

// LocationUpdatePayload
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct LocationUpdatePayload {
    matatu_id: u64,
    latitude: f64,
    longitude: f64,
    speed: f64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RecordRevenuePayload {
    sacco_id: u64,
    matatu_id: u64,
    amount: f64,
    description: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct StartTripPayload {
    matatu_id: u64,
    driver_id: u64,
    route: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct EndTripPayload {
    trip_id: u64,
    passengers: u32,
    revenue: f64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct RecordMaintenancePayload {
    matatu_id: u64,
    description: String,
    cost: f64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct RecordFuelPayload {
    matatu_id: u64,
    liters: f64,
    cost: f64,
    odometer_reading: u64,
}

// Customer Feedback Payload
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct CustomerFeedbackPayload {
    trip_id: u64,
    rating: u8,
    cleanliness: u8,
    punctuality: u8,
    safety: u8,
    comment: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
}

// Implementing Storable for SACCO
impl Storable for SACCO {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for SACCO {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Matatu
impl Storable for Matatu {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Matatu {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Driver
impl Storable for Driver {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Driver {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Expense
impl Storable for Expense {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Expense {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Revenue
impl Storable for Revenue {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Revenue {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Trip
impl Storable for Trip {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Trip {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Maintenance
impl Storable for Maintenance {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Maintenance {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for DriverPerformance
impl Storable for DriverPerformance {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for DriverPerformance {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for FuelConsumption
impl Storable for FuelConsumption {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for FuelConsumption {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Route
impl Storable for Route {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Route {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for TimeWindow
impl Storable for TimeWindow {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for TimeWindow {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for TrafficPattern
impl Storable for TrafficPattern {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for TrafficPattern {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for CustomerFeedback
impl Storable for CustomerFeedback {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for CustomerFeedback {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Schedule
impl Storable for Schedule {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Schedule {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for LocationUpdate
impl Storable for LocationUpdate {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for LocationUpdate {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for FinancialReport
impl Storable for FinancialReport {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for FinancialReport {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for ExpenseCategory
impl Storable for ExpenseCategory {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for ExpenseCategory {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for RevenueSource
impl Storable for RevenueSource {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for RevenueSource {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Memory management
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static SACCOS: RefCell<StableBTreeMap<u64, SACCO, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(10)))
        ));

    static MATATUS: RefCell<StableBTreeMap<u64, Matatu, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(11)))
        ));

    static DRIVERS: RefCell<StableBTreeMap<u64, Driver, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(12)))
        ));

    static EXPENSES: RefCell<StableBTreeMap<u64, Expense, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(13)))
        ));

    static REVENUES: RefCell<StableBTreeMap<u64, Revenue, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(14)))
        ));

    static TRIPS: RefCell<StableBTreeMap<u64, Trip, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(15)))
        ));

    static MAINTENANCE_RECORDS: RefCell<StableBTreeMap<u64, Maintenance, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(16)))
        ));

    static DRIVER_PERFORMANCE: RefCell<StableBTreeMap<u64, DriverPerformance, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(17)))
        ));

    static FUEL_RECORDS: RefCell<StableBTreeMap<u64, FuelConsumption, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(18)))
        ));

    static ROUTES: RefCell<StableBTreeMap<u64, Route, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(19)))
        ));

    static CUSTOMER_FEEDBACK: RefCell<StableBTreeMap<u64, CustomerFeedback, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(20)))
        ));

    static SCHEDULES: RefCell<StableBTreeMap<u64, Schedule, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(21)))
        ));

    static LOCATION_UPDATES: RefCell<StableBTreeMap<u64, LocationUpdate, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(22)))
        ));

    static FINANCIAL_REPORTS: RefCell<StableBTreeMap<u64, FinancialReport, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(23)))
        ));

    static EXPENSE_CATEGORIES: RefCell<StableBTreeMap<u64, ExpenseCategory, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(24)))
        ));

    static REVENUE_SOURCES: RefCell<StableBTreeMap<u64, RevenueSource, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(25)))
        ));

    static TIME_WINDOWS: RefCell<StableBTreeMap<u64, TimeWindow, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(26)))
        ));

    static TRAFFIC_PATTERNS: RefCell<StableBTreeMap<u64, TrafficPattern, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(27)))
        ));

}

// Functions

// Create SACCO
#[ic_cdk::update]
fn create_sacco(payload: CreateSACCOPayload) -> Result<SACCO, Message> {
    if payload.name.is_empty() || payload.contact.is_empty() || payload.email.is_empty() {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    let sacco_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let sacco = SACCO {
        id: sacco_id,
        name: payload.name,
        location: payload.location,
        contact: payload.contact,
        email: payload.email,
        created_at: time(),
    };

    SACCOS.with(|saccos| {
        saccos.borrow_mut().insert(sacco_id, sacco.clone());
    });

    Ok(sacco)
}

// Register Matatu
#[ic_cdk::update]
fn register_matatu(payload: RegisterMatatuPayload) -> Result<Matatu, Message> {
    if payload.plate_number.is_empty() || payload.route.is_empty() {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    let sacco_exists = SACCOS.with(|saccos| saccos.borrow().contains_key(&payload.sacco_id));
    if !sacco_exists {
        return Err(Message::NotFound("SACCO not found".to_string()));
    }

    let matatu_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let matatu = Matatu {
        id: matatu_id,
        sacco_id: payload.sacco_id,
        plate_number: payload.plate_number,
        capacity: payload.capacity,
        route: payload.route,
        status: "active".to_string(),
    };

    MATATUS.with(|matatus| {
        matatus.borrow_mut().insert(matatu_id, matatu.clone());
    });

    Ok(matatu)
}

// Register Driver
#[ic_cdk::update]
fn register_driver(payload: RegisterDriverPayload) -> Result<Driver, Message> {
    if payload.name.is_empty() || payload.license_number.is_empty() {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    let sacco_exists = SACCOS.with(|saccos| saccos.borrow().contains_key(&payload.sacco_id));
    if !sacco_exists {
        return Err(Message::NotFound("SACCO not found".to_string()));
    }

    let driver_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let driver = Driver {
        id: driver_id,
        sacco_id: payload.sacco_id,
        name: payload.name,
        license_number: payload.license_number,
        contact: payload.contact,
        assigned_matatu: None,
    };

    DRIVERS.with(|drivers| {
        drivers.borrow_mut().insert(driver_id, driver.clone());
    });

    Ok(driver)
}

// Assign Driver to Matatu
#[ic_cdk::update]
fn assign_driver_to_matatu(driver_id: u64, matatu_id: u64) -> Result<Driver, Message> {
    let matatu_exists = MATATUS.with(|matatus| matatus.borrow().contains_key(&matatu_id));
    if !matatu_exists {
        return Err(Message::NotFound("Matatu not found".to_string()));
    }

    DRIVERS.with(|drivers| {
        let mut drivers_map = drivers.borrow_mut();

        if let Some(driver) = drivers_map.get(&driver_id) {
            let mut updated_driver = driver.clone();
            updated_driver.assigned_matatu = Some(matatu_id);
            drivers_map.insert(driver_id, updated_driver.clone());
            Ok(updated_driver)
        } else {
            Err(Message::NotFound("Driver not found".to_string()))
        }
    })
}

#[ic_cdk::update]
fn start_trip(payload: StartTripPayload) -> Result<Trip, Message> {
    // Validate matatu and driver existence
    let matatu_exists = MATATUS.with(|matatus| matatus.borrow().contains_key(&payload.matatu_id));
    let driver_exists = DRIVERS.with(|drivers| drivers.borrow().contains_key(&payload.driver_id));

    if !matatu_exists || !driver_exists {
        return Err(Message::NotFound("Matatu or Driver not found".to_string()));
    }

    let trip_id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1).unwrap();
        current_value + 1
    });

    let trip = Trip {
        id: trip_id,
        matatu_id: payload.matatu_id,
        driver_id: payload.driver_id,
        start_time: time(),
        end_time: None,
        passengers: 0,
        route: payload.route,
        status: "ongoing".to_string(),
        revenue: 0.0,
    };

    TRIPS.with(|trips| trips.borrow_mut().insert(trip_id, trip.clone()));
    Ok(trip)
}

#[ic_cdk::update]
fn end_trip(payload: EndTripPayload) -> Result<Trip, Message> {
    TRIPS.with(|trips| {
        let mut trips_map = trips.borrow_mut();
        if let Some(mut trip) = trips_map.get(&payload.trip_id) {
            if trip.status != "ongoing" {
                return Err(Message::Error("Trip is not ongoing".to_string()));
            }

            trip.end_time = Some(time());
            trip.passengers = payload.passengers;
            trip.revenue = payload.revenue;
            trip.status = "completed".to_string();

            // Update driver performance
            update_driver_performance(trip.driver_id, payload.revenue);

            trips_map.insert(payload.trip_id, trip.clone());
            Ok(trip)
        } else {
            Err(Message::NotFound("Trip not found".to_string()))
        }
    })
}

#[ic_cdk::query]
fn get_driver_performance(driver_id: u64, month: u64) -> Result<DriverPerformance, Message> {
    DRIVER_PERFORMANCE.with(|performances| {
        performances
            .borrow()
            .iter()
            .find(|(_, p)| p.driver_id == driver_id && p.month == month)
            .map(|(_, p)| p.clone())
            .ok_or(Message::NotFound(
                "Performance record not found".to_string(),
            ))
    })
}

#[ic_cdk::query]
fn get_matatu_analytics(matatu_id: u64) -> Result<MatatuAnalytics, Message> {
    let total_trips = TRIPS.with(|trips| {
        trips
            .borrow()
            .iter()
            .filter(|(_, t)| t.matatu_id == matatu_id)
            .count()
    });

    let total_revenue = TRIPS.with(|trips| {
        trips
            .borrow()
            .iter()
            .filter(|(_, t)| t.matatu_id == matatu_id)
            .map(|(_, t)| t.revenue)
            .sum()
    });

    let maintenance_costs = MAINTENANCE_RECORDS.with(|records| {
        records
            .borrow()
            .iter()
            .filter(|(_, r)| r.matatu_id == matatu_id)
            .map(|(_, r)| r.cost)
            .sum()
    });

    let fuel_costs = FUEL_RECORDS.with(|records| {
        records
            .borrow()
            .iter()
            .filter(|(_, r)| r.matatu_id == matatu_id)
            .map(|(_, r)| r.cost)
            .sum()
    });

    Ok(MatatuAnalytics {
        total_trips,
        total_revenue,
        maintenance_costs,
        fuel_costs,
        net_profit: total_revenue - maintenance_costs - fuel_costs,
    })
}

// Route Optimization Functions
#[ic_cdk::update]
fn optimize_route(route_id: u64, current_time: u64) -> Result<RouteOptimization, Message> {
    ROUTES.with(|routes| {
        if let Some(route) = routes.borrow().get(&route_id) {
            let current_hour = (current_time / 3600) % 24;
            let current_day = ((current_time / 86400) % 7) as u8;

            // Find current traffic pattern
            let default_pattern = TrafficPattern::default();
            let traffic_pattern = route
                .traffic_patterns
                .iter()
                .find(|tp| {
                    let window = &tp.time_window;
                    window.day_of_week == current_day
                        && window.start_hour <= current_hour as u8
                        && window.end_hour > current_hour as u8
                })
                .unwrap_or(&default_pattern);

            // Calculate optimized route details
            let base_time = route.estimated_time as f64;
            let delay_factor = 1.0 + (traffic_pattern.congestion_level as f64 / 5.0);
            let optimized_time = (base_time * delay_factor) as u32;

            Ok(RouteOptimization {
                route_id,
                optimal_start_time: current_time + 600, // 10 minutes buffer
                estimated_duration: optimized_time,
                congestion_level: traffic_pattern.congestion_level,
                alternate_routes: vec![], // Could be implemented with actual alternatives
            })
        } else {
            Err(Message::NotFound("Route not found".to_string()))
        }
    })
}



// Customer Feedback System
#[ic_cdk::update]
fn submit_feedback(payload: CustomerFeedbackPayload) -> Result<CustomerFeedback, Message> {
    let feedback_id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1).unwrap();
        current_value + 1
    });

    let feedback = CustomerFeedback {
        id: feedback_id,
        trip_id: payload.trip_id,
        rating: payload.rating,
        cleanliness: payload.cleanliness,
        punctuality: payload.punctuality,
        safety: payload.safety,
        comment: payload.comment,
        timestamp: time(),
    };

    CUSTOMER_FEEDBACK
        .with(|feedbacks| feedbacks.borrow_mut().insert(feedback_id, feedback.clone()));

    // Update driver performance based on feedback
    update_driver_performance(feedback.trip_id, feedback.rating as f64);

    Ok(feedback)
}

// Automated Scheduling System
#[ic_cdk::update]
fn create_automated_schedule(sacco_id: u64, date: u64) -> Result<Vec<Schedule>, Message> {
    let mut schedules = Vec::new();

    // Get all available matatus and drivers
    let available_matatus = get_available_matatus(sacco_id, date);
    let available_drivers = get_available_drivers(sacco_id, date);
    let routes: Vec<Route> =
        ROUTES.with(|r| r.borrow().iter().map(|(_, route)| route.clone()).collect());

    // Create optimal schedule based on historical data and availability
    for (matatu, driver) in available_matatus.iter().zip(available_drivers.iter()) {
        for route in routes.iter() {
            let optimal_times = calculate_optimal_times(route, date);

            for (start_time, end_time) in optimal_times {
                let schedule = Schedule {
                    id: generate_id(),
                    matatu_id: matatu.id,
                    driver_id: driver.id,
                    route_id: route.id,
                    start_time,
                    end_time,
                    status: "scheduled".to_string(),
                    created_at: time(),
                };

                SCHEDULES.with(|s| s.borrow_mut().insert(schedule.id, schedule.clone()));

                schedules.push(schedule);
            }
        }
    }

    Ok(schedules)
}

// Real-time Tracking System
#[ic_cdk::update]
fn update_location(payload: LocationUpdatePayload) -> Result<LocationUpdate, Message> {
    let update_id = generate_id();
    let location_update = LocationUpdate {
        id: update_id,
        matatu_id: payload.matatu_id,
        latitude: payload.latitude,
        longitude: payload.longitude,
        speed: payload.speed,
        timestamp: time(),
    };

    LOCATION_UPDATES.with(|updates| {
        updates
            .borrow_mut()
            .insert(update_id, location_update.clone())
    });

    // Update estimated arrival times for affected schedules
    update_arrival_estimates(payload.matatu_id, &location_update);

    Ok(location_update)
}

// Financial Reporting System
#[ic_cdk::query]
fn generate_financial_report(
    sacco_id: u64,
    start_time: u64,
    end_time: u64,
) -> Result<FinancialReport, Message> {
    let revenues = calculate_total_revenues(sacco_id, start_time, end_time);
    let expenses = calculate_total_expenses(sacco_id, start_time, end_time);

    let revenue_breakdown = analyze_revenue_sources(sacco_id, start_time, end_time);
    let expense_breakdown = analyze_expense_categories(sacco_id, start_time, end_time);

    let profit_margin = if revenues > 0.0 {
        ((revenues - expenses) / revenues) * 100.0
    } else {
        0.0
    };

    let report = FinancialReport {
        id: generate_id(),
        sacco_id,
        period_start: start_time,
        period_end: end_time,
        total_revenue: revenues,
        total_expenses: expenses,
        expense_breakdown,
        revenue_breakdown,
        profit_margin,
        created_at: time(),
    };

    FINANCIAL_REPORTS.with(|reports| reports.borrow_mut().insert(report.id, report.clone()));

    Ok(report)
}

// Helper Functions

fn calculate_optimal_times(route: &Route, date: u64) -> Vec<(u64, u64)> {
    let mut optimal_times = Vec::new();

    // Calculate optimal departure times based on:
    // 1. Historical peak hours
    // 2. Traffic patterns
    // 3. Average passenger demand
    // 4. Vehicle availability

    for peak_hour in &route.peak_hours {
        let start_time = date + (peak_hour.start_hour as u64 * 3600);
        let end_time = start_time + (route.estimated_time as u64 * 60);
        optimal_times.push((start_time, end_time));
    }

    optimal_times
}

fn update_arrival_estimates(matatu_id: u64, location: &LocationUpdate) {
    SCHEDULES.with(|schedules| {
        let mut schedules_map = schedules.borrow_mut();
        let keys_to_update: Vec<u64> = schedules_map
            .iter()
            .filter_map(|(key, schedule)| {
                if schedule.matatu_id == matatu_id && schedule.status == "in_progress" {
                    Some(key)
                } else {
                    None
                }
            })
            .collect();

        for key in keys_to_update {
            if let Some(schedule) = schedules_map.get(&key) {
                let mut updated_schedule = schedule.clone(); // Clone the schedule for modification
                let new_end_time = calculate_new_arrival_time(&updated_schedule, location);
                updated_schedule.end_time = new_end_time;
                schedules_map.insert(key, updated_schedule);
            }
        }
    });
}




fn calculate_total_revenues(sacco_id: u64, start_time: u64, end_time: u64) -> f64 {
    REVENUES.with(|revenues| {
        revenues
            .borrow()
            .iter()
            .filter(|(_, r)| r.sacco_id == sacco_id && r.date >= start_time && r.date <= end_time)
            .map(|(_, r)| r.amount)
            .sum()
    })
}

fn analyze_revenue_sources(sacco_id: u64, start_time: u64, end_time: u64) -> Vec<RevenueSource> {
    // Analyze revenue by different categories:
    // 1. Regular trips
    // 2. Special routes
    // 3. Peak hour surcharges
    // 4. Additional services
    let total_revenue = calculate_total_revenues(sacco_id, start_time, end_time);

    REVENUES.with(|revenues| {
        let revenues_map = revenues.borrow();
        let mut sources = HashMap::new();

        for (_, revenue) in revenues_map.iter() {
            if revenue.sacco_id == sacco_id
                && revenue.date >= start_time
                && revenue.date <= end_time
            {
                *sources.entry(revenue.description.clone()).or_insert(0.0) += revenue.amount;
            }
        }

        sources
            .into_iter()
            .map(|(source, amount)| RevenueSource {
                source,
                amount,
                percentage: if total_revenue > 0.0 {
                    (amount / total_revenue) * 100.0
                } else {
                    0.0
                },
            })
            .collect()
    })
}

// Helper function to get get_available_matatus
fn get_available_matatus(sacco_id: u64, date: u64) -> Vec<Matatu> {
    MATATUS.with(|matatus| {
        matatus
            .borrow()
            .iter()
            .filter(|(_, m)| m.sacco_id == sacco_id && m.status == "active")
            .map(|(_, m)| m.clone())
            .collect()
    })
}

// Helper function to get get_available_drivers
fn get_available_drivers(sacco_id: u64, date: u64) -> Vec<Driver> {
    DRIVERS.with(|drivers| {
        drivers
            .borrow()
            .iter()
            .filter(|(_, d)| d.sacco_id == sacco_id)
            .map(|(_, d)| d.clone())
            .collect()
    })
}

// Helper function to calculate_new_arrival_time
fn calculate_new_arrival_time(schedule: &Schedule, location: &LocationUpdate) -> u64 {
    // Calculate new estimated arrival time based on:
    // 1. Current location
    // 2. Remaining distance
    // 3. Current traffic conditions
    // 4. Historical travel times
    let remaining_distance = 0.0; // Could be calculated based on route and current location
    let current_speed = location.speed;
    let estimated_time = remaining_distance / current_speed;

    location.timestamp + (estimated_time as u64)
}

// Helper function to calculate_total_expenses
fn calculate_total_expenses(sacco_id: u64, start_time: u64, end_time: u64) -> f64 {
    EXPENSES.with(|expenses| {
        expenses
            .borrow()
            .iter()
            .filter(|(_, e)| e.sacco_id == sacco_id && e.date >= start_time && e.date <= end_time)
            .map(|(_, e)| e.amount)
            .sum()
    })
}

// Helper function to analyze_expense_categories
fn analyze_expense_categories(
    sacco_id: u64,
    start_time: u64,
    end_time: u64,
) -> Vec<ExpenseCategory> {
    // Analyze expenses by different categories:
    // 1. Fuel
    // 2. Maintenance
    // 3. Salaries
    // 4. Insurance
    let total_expenses = calculate_total_expenses(sacco_id, start_time, end_time);

    EXPENSES.with(|expenses| {
        let expenses_map = expenses.borrow();
        let mut categories = HashMap::new();

        for (_, expense) in expenses_map.iter() {
            if expense.sacco_id == sacco_id
                && expense.date >= start_time
                && expense.date <= end_time
            {
                *categories.entry(expense.category.clone()).or_insert(0.0) += expense.amount;
            }
        }

        categories
            .into_iter()
            .map(|(category, amount)| ExpenseCategory {
                category,
                amount,
                percentage: if total_expenses > 0.0 {
                    (amount / total_expenses) * 100.0
                } else {
                    0.0
                },
            })
            .collect()
    })
}

// Helper function to update driver performance
fn update_driver_performance(driver_id: u64, trip_revenue: f64) {
    let current_month = time() / (30 * 24 * 60 * 60 * 1_000_000_000);

    DRIVER_PERFORMANCE.with(|performances| {
        let mut performances_map = performances.borrow_mut();
        let performance = performances_map
            .iter()
            .find(|(_, p)| p.driver_id == driver_id && p.month == current_month)
            .map(|(_, p)| p.clone())
            .unwrap_or_else(|| {
                // Create new performance record if none exists
                DriverPerformance {
                    id: ID_COUNTER.with(|counter| {
                        let current_value = *counter.borrow().get();
                        counter.borrow_mut().set(current_value + 1).unwrap();
                        current_value + 1
                    }),
                    driver_id,
                    month: current_month,
                    trips_completed: 0,
                    total_revenue: 0.0,
                    customer_rating: 0.0,
                    compliance_score: 100.0,
                }
            });

        let mut updated_performance = performance.clone();
        updated_performance.trips_completed += 1;
        updated_performance.total_revenue += trip_revenue;

        performances_map.insert(updated_performance.id, updated_performance);
    });
}

// Generate a new unique ID
fn generate_id() -> u64 {
    ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1).unwrap();
        current_value + 1
    })
}

// Exporting the candid interface
ic_cdk::export_candid!();
