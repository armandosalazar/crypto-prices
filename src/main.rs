fn main() {
    let mut buffer: String = String::new();

    println!("Enter a cryptocurrency: ");
    // Con el expect evitamos el match de _result
    let _ = std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");


    let price: Result<String, ureq::Error> = get_price(&buffer);
    match price {
        Ok(price) => println!("Price: ${}", price),
        Err(error) => println!("Error: {}", error)
    }
}

// &str is a reference to a string
// &String is a reference to a String
fn get_price(cryptocurrency: &str) -> Result<String, ureq::Error> {
    println!("Cryptocurrency: {}", cryptocurrency);

    let url: String = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", cryptocurrency);
    let body: String = ureq::get(&url)
        .call()?
        .into_string()?;

    let data: serde_json::Value = serde_json::from_str(&body).unwrap();
    println!("Data: {}", data);

    Ok(data["bitcoin"]["usd"].to_string())
}