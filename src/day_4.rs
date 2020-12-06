use regex::Regex;

#[derive(Hash)]
pub struct Document {
  byr: String,
  iyr: String,
  eyr: String,
  hgt: String,
  hcl: String,
  ecl: String,
  pid: String
}

pub fn solve(documents: Vec<String>) {

  let mut doc: Document;
  let mut valid_count: i16 = 0;

  for i in documents {

    doc = Document {
      byr: get_value(i.clone(), String::from("byr:")),
      iyr: get_value(i.clone(), String::from("iyr:")),
      eyr: get_value(i.clone(), String::from("eyr:")),
      hgt: get_value(i.clone(), String::from("hgt:")),
      hcl: get_value(i.clone(), String::from("hcl:")),
      ecl: get_value(i.clone(), String::from("ecl:")),
      pid: get_value(i.clone(), String::from("pid:"))
    };

    if missing_value(&doc) {
      continue
    }

    if !validate_doc(doc) {
      continue
    }
    
    valid_count += 1;

  }

  println!("{}", valid_count);

}

fn get_value(doc: String, code: String) -> String {
  for i in doc.split(|c: char| c.is_ascii_whitespace()) {
    if i.contains(&code) {
      return i.replace(&code, "").replace(" ", "")
    }
  }
  return String::from("None");
}

fn missing_value(doc: &Document) -> bool {
  return doc.byr == "None"
  ||
  doc.iyr == "None"
  ||
  doc.eyr == "None"
  ||
  doc.hgt == "None"
  ||
  doc.hcl == "None"
  ||
  doc.ecl == "None"
  ||
  doc.pid == "None";
}

fn validate_doc(doc: Document) -> bool {
  let byr: i32 = doc.byr.parse::<i32>().unwrap();
  let iyr: i32 = doc.iyr.parse::<i32>().unwrap();
  let eyr: i32 = doc.eyr.parse::<i32>().unwrap();
  let hgt: i32 = doc.hgt.trim_matches(|c: char| !c.is_numeric()).parse::<i32>().unwrap();
  let hgt_unit: String = String::from(doc.hgt.trim_matches(|c: char| c.is_numeric()));
  let hcl: String = doc.hcl;
  let ecl: String = doc.ecl;
  let pid: String = doc.pid;

  let h_colour_re = Regex::new("^#[a-f0-9]{6}$").unwrap();
  let e_colour_re = Regex::new("^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
  let p_id_re = Regex::new("^[0-9]{9}$").unwrap();

  return (byr >= 1920 && byr <= 2002)
  &&
  (iyr >= 2010 && iyr <= 2020)
  &&
  (eyr >= 2020 && eyr <= 2030)
  &&
  (validate_height(hgt, hgt_unit))
  &&
  (h_colour_re.is_match(&hcl))
  &&
  (e_colour_re.is_match(&ecl))
  &&
  (p_id_re.is_match(&pid))
}

fn validate_height(hgt: i32, hgt_unit: String) -> bool {
  return (
    hgt_unit == "in" && (hgt >= 59 && hgt <= 76)
  )
  ||
  (
    hgt_unit == "cm" && (hgt >= 150 && hgt <= 193)
  );
}