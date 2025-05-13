// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        // can also replace "Order" with "Self" in the line above
        Self::validate_product_name(&product_name);
        Self::validate_quantity(&quantity);
        Self::validate_price(&unit_price);

        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

    fn validate_product_name(name: &String) {
        if name.is_empty() {
            panic!("name cannot be empty")
        }

        if name.len() > 300 {
            panic!("name cannot be longer than 300 bytes")
        }
    }

    fn validate_quantity(quantity: &u32) {
        if quantity <= &0 {
            panic!("Quantity is not greater than 0")
        }
    }

    fn validate_price(price: &u32) {
        if price <= &0 {
            panic!("Price is not greater than 0")
        }
    }

    // this is a getter/accessor method
    // just to provide a public method to read-only access the fields (the struct field remains private)
    // so say a user wants to know the product name, they can call: order.product_name() where order is the struct
    // they cannot access the (private) field directly using order.product_name, because the fields are private
    // hence this is called getters (Rustbook Sec 5.3) / accessor method
    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn total(&self) -> u32 {
        // self itself is an Order struct, so we can access the fields directly
        // similar to Rustbook section 5.3 https://rust-book.cs.brown.edu/ch05-03-method-syntax.html
        // where when calculate the area, just use self.width*self.height (i.e., no need to put width and height as inputs to the method/function)
        self.quantity * self.unit_price
    }

    pub fn set_product_name(&mut self, product_name: String) {
        Self::validate_product_name(&product_name);

        self.product_name = product_name
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        Self::validate_quantity(&quantity);

        self.quantity = quantity
    }

    pub fn set_unit_price(&mut self, unit_price: u32) {
        Self::validate_price(&unit_price);

        self.unit_price = unit_price
    }
}
