use std::collections::HashMap;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

struct URLShortner {
    urls:HashMap<String, String>
}

impl URLShortner {
    fn new () -> Self {
        Self {
            urls:HashMap::new(),
        }
    }

    fn generate_url_code(&mut self) -> String {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        format!("{:x}", timestamp)
    }

    fn generate_short_url(&mut self, long_url:&str) -> String {
        let short_code = self.generate_url_code();
        self.urls.insert(short_code.clone(), long_url.to_string());
        short_code
    }

    fn get_full_url(&self, short_code:&str) -> String  {
        let url = self.urls.get(short_code).map(|url| url.as_str());
        match url {
            Some(url) => url.to_string(),
            None => "".to_string()
        }
    }
}

fn main() {

    let mut shortner = URLShortner::new();

    loop {
        println!("1. Shorten URL");
        println!("2. Retrieve URL");
        println!("3. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice:u32 = choice.trim().parse().unwrap();

        match choice {
            1 => {
                print!("Enter long URL");
                io::stdout().flush().unwrap();
                let mut long_url = String::new();
                io::stdin().read_line(&mut long_url).unwrap();
                let short_code = shortner.generate_short_url(&long_url);
                println!("{}", short_code);
            }

            2 => {
                print!("Enter url code");
                io::stdout().flush().unwrap();
                let mut url_code = String::new();
                io::stdin().read_line(&mut url_code).unwrap();
                let url = shortner.get_full_url(&url_code.trim());
                if !url.is_empty() {
                    println!("Original url {}", url);
                } else {
                    println!("No url found");
                }
            }

            3 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}
