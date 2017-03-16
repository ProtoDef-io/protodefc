use ::std::io::Read;
use ::std::iter;

#[derive(Debug)]
pub struct TestCase {
    pub name: String,
    pub json_type: ::json::JsonValue,
    pub values: Vec<TestCaseValue>,
}

#[derive(Debug)]
pub struct TestCaseValue {
    pub json_value: String,
    pub serialized: Vec<u8>,
}

pub fn cases() -> Vec<TestCase> {
    ::std::fs::read_dir("protodef_spec/test/").unwrap()
        .map(|r| r.unwrap())
        .filter(|e| e.path().extension().map(|i| i == "json").unwrap_or(false))
        .filter(|e| !e.path().file_stem().unwrap().to_str().unwrap().contains("schema"))
        .flat_map(|e| {
            let mut file = ::std::fs::File::open(e.path()).unwrap();
            let mut string = String::new();
            file.read_to_string(&mut string).unwrap();

            let parsed = ::json::parse(&string).unwrap();
            assert!(parsed.is_array());

            let file_path = e.path();
            let file_name_raw = file_path.file_stem().unwrap();
            let file_name = file_name_raw.to_str().unwrap();

            let mut cases: Vec<TestCase> = Vec::new();

            for top_case in parsed.members() {
                if top_case.has_key("subtypes") {
                    assert!(top_case["subtypes"].is_array());
                    let top_case_name = top_case["type"].as_str().unwrap();

                    for case in top_case["subtypes"].members() {
                        cases.push(json_to_case(case, format!("{}.{}", file_name, top_case_name)));
                    }
                } else {
                    cases.push(json_to_case(top_case, file_name.into()));
                }
            }

            cases
        }).collect()
}


fn json_to_case(case: &::json::JsonValue, base_name: String) -> TestCase {
    let values_json = &case["values"];
    assert!(values_json.is_array());
    let values: Vec<TestCaseValue> = values_json.members().map(|value| {
        let json_buf = &value["buffer"];
        assert!(json_buf.is_array());
        let binary: Vec<u8> = json_buf.members().map(|t| {
            u8::from_str_radix(&t.as_str().unwrap()[2..], 16).unwrap()
        }).collect();

        let json_value = ::json::stringify(value["value"].clone());

        TestCaseValue {
            json_value: json_value,
            serialized: binary,
        }
    }).collect();

    let name = if case.has_key("description") {
        format!("{}: {}", base_name, case["description"].as_str().unwrap())
    } else {
        base_name
    };

    TestCase {
        name: name,
        json_type: case["type"].clone(),
        values: values,
    }
}

#[cfg(test)]
mod tests {
    use super::cases;

    #[test]
    fn gen_spec_test_cases() {
        cases();
    }

}
