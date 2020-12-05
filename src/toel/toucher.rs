use reqwest;
use colored::*;
use tokio::runtime::Runtime;
use futures::future::join_all;

#[derive(Debug)]
pub struct HostEntity {
    domain: String,
    protocol: String
}

impl HostEntity {
    fn get_host(&self) -> String {
        let mut host = self.protocol.clone();
        host.push_str("://");
        host.push_str(&self.domain);
        
        host.to_string()
    }
}

pub fn touch(subdomains: Vec<String>) {

    let mut rt = Runtime::new().unwrap();
    let client = reqwest::Client::new();
    
    let mut tasks = vec![];
    for domain in subdomains {
        for protocol in ["http", "https"].iter() {
            let c = client.clone();
            let host = HostEntity{ domain: domain.to_string(), protocol: protocol.to_string() };
                
            let task = rt.spawn(touch_public_port(c, host));
            tasks.push(task);
        }
    }

    rt.block_on(join_all(tasks));
}

pub async fn touch_public_port(client: reqwest::Client, host: HostEntity) {

    let response = client
        .get(&host.get_host()).send().await;

    match response {
        Ok(result) => {
            let status = result.status().to_string().to_lowercase();
            let result = is_live(status, host.get_host());
            match result {
                Ok(content) => {
                    println!("{}", content.green());
                }
                Err(_) => {}
            }
            
        } 
        Err(_) => {} 
    }
}

fn is_live(status: String, domain: String) -> Result<String, &'static str> {
    let mut line: String = "[+] ".to_string();
    line.push_str(&domain);

    if status == "200 ok" {
        return Ok(line);
    }

    Err("")
}


#[cfg(test)]
mod toucher_test {

    #[test]
    fn is_live_test() {
        let result = super::is_live("200 ok".to_string(), "local.host".to_string()).unwrap();
        assert_eq!(result, "[+] local.host");
    }

    #[test]
    fn is_not_live_test() {
        let result = super::is_live("".to_string(), "local.host".to_string());
        match result {
            Ok(content) => {
                assert_eq!(content, "".to_string());
            }
            Err(_) => {}
        }
    }
}