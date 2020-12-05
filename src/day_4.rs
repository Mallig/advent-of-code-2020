//mod input_formatter;
#[derive(Hash)]
pub struct Document {
  byr: Option<String>,
  iyr: Option<String>,
  eyr: Option<String>,
  hgt: Option<String>,
  hcl: Option<String>,
  ecl: Option<String>,
  pid: Option<String>,
  cid: Option<String>
}

pub fn solve(documents: Vec<String>) {

  let mut doc: Document;
  let mut docs: Vec<Document> = Vec::new();

  for i in documents {

    // for j in i.split(|c: char| c.is_ascii_whitespace()) {
        
        doc = Document {
          byr: get_value(i, String::from("byr:")),
          iyr: None, //get_value(i, String::from("iyr:")),
          eyr: None, //get_value(i, String::from("eyr:")),
          hgt: None, //get_value(i, String::from("hgt:")),
          hcl: None, //get_value(i, String::from("hcl:")),
          ecl: None, //get_value(i, String::from("ecl:")),
          pid: None, //get_value(i, String::from("pid:")),
          cid: None, //get_value(i, String::from("cid:")),
        };

        println!("{}", doc.byr.unwrap());

        // docs.push(doc);


        break;
    // }
  }
}

fn get_value(doc: String, code: String) -> Option<String> {
  for i in doc.split(|c: char| c.is_ascii_whitespace()) {
    if i.contains(&code) {
      return Some(i.replace(&code, "").replace(" ", ""))
    }
  }
  return None;
}