use reqwest;
use reqwest::blocking::Response;
use xml::reader::{EventReader, XmlEvent};

const ENDPOINT: &str = "https://crt.sh/atom?q=";

#[derive(Default, Debug)]
pub struct Render {
    state: String,
    pub summaries: Vec<String>,
    pub subdomains: Vec<String>
}


pub fn get_it(domain: &String) -> Result<reqwest::blocking::Response, reqwest::Error> {
    
    let mut url: String = ENDPOINT.to_string();
    url.push_str(domain);
    reqwest::blocking::get(&url)

}


pub fn summary_collector(response: Response) -> Render {
    let content: String = response.text().unwrap();
    
    let mut render: Render = Render::default();
    
    let parse = EventReader::from_str(&content);
    for elm in parse {
        match elm {
            Ok(XmlEvent::StartElement {name, ..}) => {
                render.state = name.local_name.to_string()
            }
            Ok(XmlEvent::Characters(text)) => {
                if render.state == "summary" {
                    render.summaries.push(text)
                }
            }
            Err(_) => {}
            _ => {}
        }
    }
    render
}
