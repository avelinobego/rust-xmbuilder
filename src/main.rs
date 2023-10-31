mod builder;

fn main() {
    println!("Hello, world!");

    let mut e = builder::Element::new("eSocial");
    e.set_value("Avelino");
    for value in 1..10 {
        let mut e2 = builder::Element::new("evtTrablhador");
        e2.set_value("Trabalhador");
        
        e2.add_attrib("id".to_string(), value.to_string());
        e.add_element(*e2);
    }
    e.build_xml();

    println!("{}", e);

}
