use std::collections::HashMap;

#[derive(Debug)]
struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Self {
        Self {
            departments: HashMap::new(),
        }
    }

    fn add_employee(&mut self, name: &str, department: &str) {
        let dept = self
            .departments
            .entry(department.to_string())
            .or_insert(Vec::new());
        dept.push(name.to_string());
    }

    fn get_all_in_department(&self, department: &str) -> Option<&Vec<String>> {
        self.departments.get(department)
    }

    fn get_all(&self) {
        for (department, employees) in &self.departments {
            println!("{department}");
            let mut sorted_employees = employees.clone();
            sorted_employees.sort();
            for employee in sorted_employees {
                println!(" - {}", employee);
            }
        }
    }
}

fn main() {
    let mut company = Company::new();
    company.add_employee("Sally", "Engineering");
    company.add_employee("Amanda", "Engineering");
    company.add_employee("Trevor", "Engineering");
    company.add_employee("Bethany", "Sales");
    company.add_employee("Carver", "Sales");
    company.add_employee("Amir", "Sales");

    println!("{:?}", company);
    println!("{:?}", company.get_all_in_department("Engineering"));
    println!("{:?}", company.get_all());
}
