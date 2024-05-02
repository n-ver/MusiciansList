use thirtyfour::prelude::*;
use tokio::time::Duration;
use std::env;
use std::fmt;
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::error::Error;
use tokio;
use mongodb::bson::doc;
use mongodb::Database;
use mongodb::Collection;
use bson::Document;


struct MusData {
    id: String,
    name: String,
    instruments: String,
    years: String,
    sessions: i32,
}

impl fmt::Display for MusData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let str = format!(
            "{} {} {} {} {}",
            self.id,
            self.name,
            self.instruments,
            self.years,
            self.sessions
        );
        write!(fmt, "{}", str) 
        // Ok(())
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("starting database");
    let client_uri = "FIX ME";
    // let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!"); not working

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;
    let database = client.database("Jazz_Musicians");
    let v = vec![
    MusData {
        id : "M258350".to_string(),
        name : "".to_string(),
        instruments : " ".to_string(),
        years : "1976-2023".to_string(),
        sessions : 5,
    },
    MusData {
        id : "M75499".to_string(),
        name : "\"Blackie\"".to_string(),
        instruments : "d".to_string(),
        years : "1926-1927".to_string(),
        sessions : 6,
    },
    MusData {
        id : "M158245".to_string(),
        name : "\"Wesley \"G\"\"".to_string(),
        instruments : "g".to_string(),
        years : "1999".to_string(),
        sessions : 3,
    },
    MusData {
        id : "M75498".to_string(),
        name : "\"Tubi\"".to_string(),
        instruments : "tu".to_string(),
        years : "1926-1927".to_string(),
        sessions : 6,
    },
    MusData {
        id : "M221030".to_string(),
        name : "Willy ---".to_string(),
        instruments : "tp".to_string(),
        years : "1946".to_string(),
        sessions : 3,
    },
    MusData {
        id : "M82320".to_string(),
        name : "\"Chief\" ...".to_string(),
        instruments : "tu,b".to_string(),
        years : "1923-1944".to_string(),
        sessions : 7,
    },
    MusData {
        id : "M106354".to_string(),
        name : "\"Shaky Walter\" ...".to_string(),
        instruments : "hca".to_string(),
        years : "1927-1928".to_string(),
        sessions : 5,
    },
    MusData {
        id : "M66369".to_string(),
        name : "Billy ...".to_string(),
        instruments : "cl,as".to_string(),
        years : "1924".to_string(),
        sessions : 4,
    },
    MusData {
        id : "M20520".to_string(),
        name : "George ...".to_string(),
        instruments : "tb,b,vcl".to_string(),
        years : "1948-1980".to_string(),
        sessions : 7,
    },
    MusData {
        id : "M66408".to_string(),
        name : "Henk ...".to_string(),
        instruments : "cl,ts".to_string(),
        years : "1941".to_string(),
        sessions : 3,
    },
    MusData {
        id : "M220108".to_string(),
        name : "Jim ...".to_string(),
        instruments : "bj".to_string(),
        years : "1925".to_string(),
        sessions : 4,
    },
];

    // scraper().await;
    for data in v {
        upload(&database, data).await;
    };
    Ok(())
}
// Gathers data
async fn scraper() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", &caps).await?;

    // Navigate to log in page

    driver.get("http://www.library.illinois.edu/proxy/go.php?url=http://www.lordisco.com/tjd/CoverFrame").await?;
    // let it load
    tokio::time::sleep(Duration::from_secs(1)).await;

    let email_form = driver.find_element(By::Id("i0116")).await?;
    
    // Find element from element.
    // let elem_text = elem_form.find_element(By::Id("searchInput")).await?;

    // Type in the search terms.
    let UIUC_User : String = env::var("UIUC_User").unwrap_or_else(|_| "Unknown user".to_string());
    
    email_form.send_keys(UIUC_User).await?;
    
    // Click the next button.
    let next_button = driver.find_element(By::Id("idSIButton9")).await?;
    next_button.click().await?;

    // Type in password
    let UIUC_pass : String = env::var("UIUC_Pass").unwrap_or_else(|_| "Unknown user".to_string());
    
    driver.find_element(By::Id("i0118")).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let pass_form = driver.find_element(By::Id("i0118")).await?;
    pass_form.send_keys(UIUC_pass).await?;
    driver.find_element(By::Id("idSIButton9")).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let next_button2 = driver.find_element(By::Id("idSIButton9")).await?;
    next_button2.click().await?;
    // Look for header to implicitly wait for the page to load.
    // driver.find_element(By::ClassName("firstHeading")).await?;
    // assert_eq!(driver.title().await?, "Selenium - Wikipedia");

    tokio::time::sleep(Duration::from_secs(5)).await;
    // let search_button = WebDriverWait::new(&driver, std::time::Duration::from_secs(10))
    //     .until(|d| d.find_element(By::Css("div.musician #searchButtonId")));
    // search_button.click().await?;
    let search_button = driver.find_element(By::Css("a[href='MusicianSearch?dest=MusicianIndex'] > .nav-button.musician-element")).await?;
    search_button.click().await?;
    let search_button = driver.find_element(By::Css("input[name='action2'][value='Search']")).await?;
    search_button.click().await?;
    let table = driver.find_element(By::Css("table.index")).await?;
    let rows = table.find_elements(By::Css("tr")).await?;
    let mut musicians_with_more_than_two_sessions : Vec<MusData> = Vec::new();

    for row in rows.iter().skip(1) {  // Skip header row
        let cells = row.find_elements(By::Css("td")).await?;
        if cells.len() > 2 { // Ensure there are enough columns in the row
            let id = row.get_attribute("id").await?.unwrap_or_else(|| "id failed".to_string());
            let musician_name = cells[0].text().await?; // Index might need adjustment
            let years = cells[1].text().await?;
            let instruments = cells[2].text().await?;
            let session_count: i32 = cells[3].text().await?.parse().unwrap_or(0); // Index might need adjustment

            if session_count > 2 {
                musicians_with_more_than_two_sessions.push(MusData {
                    id: id,
                    name : musician_name,
                    instruments : instruments,
                    years: years,
                    sessions: session_count,
                });
            }
        }
    }

    // Output the musicians
    println!("let v = vect![");
    for musician in musicians_with_more_than_two_sessions {
        println!("MusData {{ \nid : \"{}\",\nname : \"{}\",\ninstruments : \"{}\",\nyears : \"{}\",\nsessions : {},\n}},", musician.id, musician.name, musician.instruments, musician.years, musician.sessions);
    }
    println!("];");
    Ok(())
}

// Uploads to database
async fn upload(database: &Database, data: MusData) -> Result<(), Box<dyn Error>> {
    // filters into correct collection
    if data.sessions < 50 {
        let collection = database.collection("0025");
        insert_or_not(collection, data).await?;
    } else if data.sessions < 100 {
        let collection = database.collection("0050");
        insert_or_not(collection, data).await?;
    } else if data.sessions < 200 {
        let collection = database.collection("0100");
        insert_or_not(collection, data).await?;
    } else if data.sessions < 500 {
        let collection = database.collection("0200");
        insert_or_not(collection, data).await?;
    } else if data.sessions < 1000 {
        let collection = database.collection("0500");
        insert_or_not(collection, data).await?;
    } else {
        let collection = database.collection("1000");
        insert_or_not(collection, data).await?;
    }
    Ok(())
}

// mongodb+srv://Jazz_Musician_Scraper:TzErtYpD74ueEQ1I@cluster0.wyv1qyo.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0

// inserts if not already in database
async fn insert_or_not(collection: Collection<Document>, data: MusData)  -> Result<(), Box<dyn Error>> { 
    println!("insert or not running");
    let already_exists: Option <Document> = collection.find_one(
        doc! {
              "id": data.id.clone(),
        },
        None,
     ).await?;
     println!("doc made troubleshooting");
     if already_exists.is_none() { // entry does not exist
        let new_doc = doc! {
            "id" : data.id,
            "name" : data.name,
            "instruments" : data.instruments,
            "years active" : data.years,
            "session count" : data.sessions,
        };
    let insert_result = collection.insert_one(new_doc.clone(), None).await?;
     } 
     Ok(()) // didn't exist
}
// cedar walton bolivia
// driftin
// flintstones
// the sequel ??? miller