
use regex::Regex;

mod collector;
pub mod toucher;


pub fn run(domain: String) -> Result<(), &'static str> {
    if domain.len() <= 0 {
        return Err("Domain name is empty")
    }

    println!("\n[->] Trying to enumerate subdomain...\n");

    let response = collector::get_it(&domain);  
    match response {
        Ok(content) => {
            let render = collector::summary_collector(content);
            let subdomains = domain_extractor(&render.summaries);
            println!("[->] Subdomain successfully collected,\n     start trying to check for online host only..\n");

            toucher::touch(subdomains);

            println!("\n[->] Wis rampung dab, chek'en wae hasile..\n     nek error, debug'en dewe sek wae yo.. \n");
        }
        Err(err) => {
            println!("\n[-] {}\n", err)
        }
    }

    Ok(())
}


fn domain_extractor(summaries: &Vec<String>) -> Vec<String> {
    let re = Regex::new(r"[\w+-]\.?[\w-]+\.[\w-]+").unwrap();
    let mut subdomains: Vec<String> = vec![];

    for summary in summaries {
        let result = summary.replace("\\n"," ");
        for domain in re.captures_iter(&result) {
            let subdomain = &domain[0].to_string();
            if !subdomains.contains(&subdomain) {
                subdomains.push(subdomain.to_string());
            }
        }
    }
    subdomains
}

#[cfg(test)]
mod mod_test {
    #[test]
    fn parse_domain() {
        let summaries: Vec<String> = vec!["domain.com asdsadasdajkl asdsadjk asdasda domain.com".to_string()];
        let results = super::domain_extractor(&summaries);
    
        assert_eq!(results, vec!["domain.com".to_string()]);
    }
    
    #[test]
    fn parse_domain_with_new_line() {
        let summaries: Vec<String> = vec!["domain.com\nasdsadasdajkl asdsadjk asdasda\ndomain-2.com\n".to_string()];
        let result = super::domain_extractor(&summaries);
    
        assert_eq!(result, vec!["domain.com", "domain-2.com"]);
    }
}