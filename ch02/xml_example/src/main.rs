use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Default)]
struct Product {
    id: u32,
    category: String,
    name: String,
}

#[derive(Debug, Default)]
struct Sale {
    id: String,
    product_id: u32,
    date: i64,
    quantity: f64,
    unit: String,
}

#[derive(Debug)]
struct SalesAndProducts {
    products: Vec<Product>,
    sales: Vec<Sale>,
}

///*
//#[derive(Copy, Clone)]
enum LocationItem {
    Other,
    InProduct,
    InSale,
}
enum LocationProduct {
    Other,
    InId,
    InCategory,
    InName,
}
enum LocationSale {
    Other,
    InId,
    InProductId,
    InDate,
    InQuantity,
    InUnit,
}
//*/
fn main() {
    let file = std::fs::File::open("../data/sales.xml").unwrap();
    let file = std::io::BufReader::new(file);
    let mut product: Product = Default::default();
    let mut sale: Sale = Default::default();
    let parser = EventReader::new(file);
    let mut location_item = LocationItem::Other;
    let mut location_product = LocationProduct::Other;
    let mut location_sale = LocationSale::Other;
    for event in parser {
//        println!("E: {:?}", event);
        match &location_item {
            LocationItem::Other => match event {
                Ok(XmlEvent::StartElement { ref name, .. })
                    if name.local_name == "product"
                => {
                    location_item = LocationItem::InProduct;
                    location_product = LocationProduct::Other;
                    product = Default::default();
                    //println!("Enter product");
                },
                Ok(XmlEvent::StartElement { ref name, .. })
                    if name.local_name == "sale"
                => {
                    location_item = LocationItem::InSale;
                    location_sale = LocationSale::Other;
                    sale = Default::default();
                    //println!("Enter sale");
                },
                _ => {},
            },
            LocationItem::InProduct => match &location_product {
                LocationProduct::Other =>
                    match event {
                        Ok(XmlEvent::StartElement { ref name, .. })
                            if name.local_name == "id" => {
                            location_product = LocationProduct::InId;
                            //println!("Enter product.id");
                        },
                        Ok(XmlEvent::StartElement { ref name, .. })
                            if name.local_name == "category" => {
                            location_product = LocationProduct::InCategory;
                            //println!("Enter product.category");
                        },
                        Ok(XmlEvent::StartElement { ref name, .. })
                            if name.local_name == "name" => {
                            location_product = LocationProduct::InName;
                            //println!("Enter product.name");
                        },
                        Ok(XmlEvent::EndElement { ref name, .. }) => {
                            location_item = LocationItem::Other;
                            println!("  Exit product: {:?}", product);
                        },
                        _ => {},
                    },
                LocationProduct::InId => match event {
                    Ok(XmlEvent::Characters ( characters ))
                    => {
                        product.id = characters.parse::<u32>().unwrap();
                        println!("Got product.id: {}.", characters);
                    },
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_product = LocationProduct::Other;
                        //println!("Exit product.id");
                    },
                    _ => {},
                },
                LocationProduct::InCategory => match event {
                    Ok(XmlEvent::Characters ( characters ))
                    => {
                        product.category = characters.clone();
                        println!("Got product.category: {}.", characters);
                    },
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_product = LocationProduct::Other;
                        //println!("Exit product.category");
                    },
                    _ => {},
                },
                LocationProduct::InName => match event {
                    Ok(XmlEvent::Characters ( characters ))
                    => {
                        product.name = characters.clone();
                        println!("Got product.name: {}.", characters);
                    },
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_product = LocationProduct::Other;
                        //println!("Exit product.name");
                    },
                    _ => {},
                },
            },
            LocationItem::InSale => match &location_sale {
                LocationSale::Other => match event {
                    Ok(XmlEvent::StartElement { ref name, .. })
                        if name.local_name == "id" => {
                        location_sale = LocationSale::InId;
                        //println!("Enter sale.id");
                    },
                    Ok(XmlEvent::StartElement { ref name, .. })
                        if name.local_name == "product-id" => {
                        location_sale = LocationSale::InProductId;
                        //println!("Enter sale.product-id");
                    },
                    Ok(XmlEvent::StartElement { ref name, .. })
                        if name.local_name == "date" => {
                        location_sale = LocationSale::InDate;
                        //println!("Enter sale.date");
                    },
                    Ok(XmlEvent::StartElement { ref name, .. })
                        if name.local_name == "quantity" => {
                        location_sale = LocationSale::InQuantity;
                        //println!("Enter sale.quantity");
                    },
                    Ok(XmlEvent::StartElement { ref name, .. })
                        if name.local_name == "unit" => {
                        location_sale = LocationSale::InUnit;
                        //println!("Enter sale.unit");
                    },
                    Ok(XmlEvent::EndElement { ref name, .. })
                        if name.local_name == "sale" => {
                        location_item = LocationItem::Other;
                        println!("  Exit sale: {:?}", sale);
                    },
                    _ => {},
                },
                LocationSale::InId => match event {
                    Ok(XmlEvent::Characters ( characters ))
                    => {
                        sale.id = characters.clone();
                        println!("Got sale.id: {}.", characters);
                    },
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_sale = LocationSale::Other;
                        //println!("Exit sale.id");
                    },
                    _ => {},
                },
                LocationSale::InProductId => match event {
                    Ok(XmlEvent::Characters ( characters ))
                    => {
                        sale.product_id = characters.parse::<u32>().unwrap();
                        println!("Got sale.product-id: {}.", characters);
                    },
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_sale = LocationSale::Other;
                        //println!("Exit sale.product-id");
                    },
                    _ => {},
                },
                LocationSale::InDate => match event {
                    Ok(XmlEvent::Characters ( characters ))
                    => {
                        sale.date = characters.parse::<i64>().unwrap();
                        println!("Got sale.date: {}.", characters);
                    },
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_sale = LocationSale::Other;
                        //println!("Exit sale.date");
                    },
                    _ => {},
                },
                LocationSale::InQuantity => match event {
                    Ok(XmlEvent::Characters ( characters ))
                    => {
                        sale.quantity = characters.parse::<f64>().unwrap();
                        println!("Got sale.quantity: {}.", characters);
                    },
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_sale = LocationSale::Other;
                        //println!("Exit sale.quantity");
                    },
                    _ => {},
                },
                LocationSale::InUnit => match event {
                    Ok(XmlEvent::Characters ( characters ))
                    => {
                        sale.unit = characters.clone();
                        println!("Got sale.unit: {}.", characters);
                    },
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_sale = LocationSale::Other;
                        //println!("Exit sale.unit");
                    },
                    _ => {},
                },
            },
        }
        // At StartElement product enters a product

        /*
        match location {
            Location::AtRoot =>
                match event {
                    Ok(XmlEvent::StartElement { ref name, .. }) =>
                        if name.local_name == "person" {
                            location = Location::InPerson;
                        },
                    _ => (),
                },
            Location::InPerson =>
                match event {
                    Ok(XmlEvent::StartElement { ref name, .. }) =>
                        //location = match name.local_name.as_ref() {
                        location = match name.local_name.as_str() {
                            "name" => Location::InName,
                            "child" => Location::InChild,
                            "level" => Location::InLevel,
                            _ => Location::InOtherPersonalData,
                        },
                    Ok(XmlEvent::EndElement { .. }) =>
                        location = Location::AtRoot,
                    _ => (),
                },
            Location::InName =>
                match event {
                    Ok(XmlEvent::Characters ( characters )) =>
                        p.name = characters,
                    Ok(XmlEvent::EndElement { .. }) =>
                        location = Location::InPerson,
                    _ => (),
                },
            Location::InChild =>
                match event {
                    Ok(XmlEvent::Characters ( characters )) =>
                        p.children.push(characters),
                    Ok(XmlEvent::EndElement { .. }) =>
                        location = Location::InPerson,
                    _ => (),
                },
            Location::InLevel =>
                match event {
                    Ok(XmlEvent::Characters ( characters )) =>
                        p.level = characters.parse().unwrap(),
                    Ok(XmlEvent::EndElement { .. }) =>
                        location = Location::InPerson,
                    _ => (),
                },
            Location::InOtherPersonalData =>
                match event {
                    Ok(XmlEvent::EndElement { .. }) =>
                        location = Location::InPerson,
                    _ => (),
                },
        }
        */
    }
}