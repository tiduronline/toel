mod toel;

fn banner() -> &'static str {
r#"
// ================
    Please set target domain by passing an option -d followed by domain
    Example: 
        $ toel -d tiduronline.com
// ================
"#
}


fn parse_arg(args: &Vec<String>) -> String {
    if args.len() < 2 {
        println!("{}", banner());
        return "".to_string();
    }
    return args[2].to_string()
}


fn main() -> Result<(), &'static str>{
    let domain = parse_arg(&std::env::args().collect());
    
    match toel::run(domain) {
        Ok(_) => {}
        Err(_) => {}
    };
    
    Ok(())
}


