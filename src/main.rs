use std::io::{self, Write};

// Bill structure
#[derive(Debug, Clone)]
struct Bill {
    id: u32,
    name: String,
    amount: f64,
}

// Main tracker
struct BillTracker {
    bills: Vec<Bill>,
    next_id: u32,
}

impl BillTracker {
    fn new() -> Self {
        BillTracker {
            bills: Vec::new(),
            next_id: 1,
        }
    }
    
    // Add a new bill
    fn add_bill(&mut self, name: String, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Amount must be positive".to_string());
        }
        if name.trim().is_empty() {
            return Err("Bill name cannot be empty".to_string());
        }
        
        let bill = Bill {
            id: self.next_id,
            name,
            amount,
        };
        self.bills.push(bill);
        self.next_id += 1;
        Ok(())
    }
    
    // View all bills
    fn view_bills(&self) {
        if self.bills.is_empty() {
            println!("\nNo bills found!\n");
            return;
        }
        
        println!("\n=== Your Bills ===");
        for bill in &self.bills {
            println!("  {}. {} - ${:.2}", bill.id, bill.name, bill.amount);
        }
        println!("==================\n");
    }
    
    // Remove a bill by ID
    fn remove_bill(&mut self, id: u32) -> Option<Bill> {
        let position = self.bills.iter().position(|bill| bill.id == id);
        match position {
            Some(index) => Some(self.bills.remove(index)),
            None => None,
        }
    }
    
    // Update a bill
    fn update_bill(&mut self, id: u32, new_name: Option<String>, new_amount: Option<f64>) -> Result<(), String> {
        let bill = self.bills.iter_mut().find(|bill| bill.id == id);
        
        match bill {
            Some(b) => {
                if let Some(name) = new_name {
                    if name.trim().is_empty() {
                        return Err("Name cannot be empty".to_string());
                    }
                    b.name = name;
                }
                if let Some(amount) = new_amount {
                    if amount <= 0.0 {
                        return Err("Amount must be positive".to_string());
                    }
                    b.amount = amount;
                }
                Ok(())
            }
            None => Err(format!("Bill with ID {} not found", id)),
        }
    }
    
    // Calculate total of all bills
    fn total_bills(&self) -> f64 {
        self.bills.iter().map(|bill| bill.amount).sum()
    }
}

// Helper function to read input
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Helper to read amount
fn read_amount(prompt: &str) -> Option<f64> {
    let input = read_input(prompt);
    match input.parse::<f64>() {
        Ok(amount) if amount > 0.0 => Some(amount),
        _ => {
            println!("Invalid amount! Please enter a positive number.");
            None
        }
    }
}

fn main() {
    let mut tracker = BillTracker::new();
    
    loop {
        // Display menu
        println!("\n== Manage Bills ==");
        println!("1. Add bill");
        println!("2. View bills");
        println!("3. Remove bill");
        println!("4. Update bill");
        println!("5. Bill total");
        println!("6. Exit");
        println!("------------------");
        
        let choice = read_input("Enter selection: ");
        
        match choice.as_str() {
            "1" => {
                // Add bill
                let name = read_input("Enter bill name: ");
                if let Some(amount) = read_amount("Enter amount: $") {
                    match tracker.add_bill(name, amount) {
                        Ok(()) => println!("Bill added successfully!"),
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }
            
            "2" => {
                // View bills
                tracker.view_bills();
            }
            
            "3" => {
                // Remove bill
                if tracker.bills.is_empty() {
                    println!("No bills to remove!");
                    continue;
                }
                
                tracker.view_bills();
                let id_input = read_input("Enter bill ID to remove: ");
                
                match id_input.parse::<u32>() {
                    Ok(id) => {
                        match tracker.remove_bill(id) {
                            Some(bill) => println!("Removed: {} - ${:.2}", bill.name, bill.amount),
                            None => println!("Bill with ID {} not found", id),
                        }
                    }
                    Err(_) => println!("Invalid ID!"),
                }
            }
            
            "4" => {
                // Update bill
                if tracker.bills.is_empty() {
                    println!("No bills to update!");
                    continue;
                }
                
                tracker.view_bills();
                let id_input = read_input("Enter bill ID to update: ");
                
                match id_input.parse::<u32>() {
                    Ok(id) => {
                        println!("(Press Enter to keep current value)");
                        let current_bill = tracker.bills.iter().find(|b| b.id == id);
                        
                        if let Some(bill) = current_bill {
                            let new_name = read_input(&format!("New name [{}]: ", bill.name));
                            let new_name = if new_name.is_empty() { None } else { Some(new_name) };
                            
                            println!("Current amount: ${:.2}", bill.amount);
                            let amount_input = read_input("New amount: $");
                            let new_amount = if amount_input.is_empty() {
                                None
                            } else {
                                match amount_input.parse::<f64>() {
                                    Ok(a) if a > 0.0 => Some(a),
                                    _ => {
                                        println!("Invalid amount, keeping original");
                                        None
                                    }
                                }
                            };
                            
                            match tracker.update_bill(id, new_name, new_amount) {
                                Ok(()) => println!("Bill updated successfully!"),
                                Err(e) => println!("Error: {}", e),
                            }
                        } else {
                            println!("Bill not found!");
                        }
                    }
                    Err(_) => println!("Invalid ID!"),
                }
            }
            
            "5" => {
                // Bill total
                let total = tracker.total_bills();
                println!("\nTotal of all bills: ${:.2}", total);
                if !tracker.bills.is_empty() {
                    println!("Average bill: ${:.2}", total / tracker.bills.len() as f64);
                }
            }
            
            "6" => {
                println!("Goodbye!");
                break;
            }
            
            _ => println!("Invalid selection! Please choose 1-6"),
        }
    }
}
