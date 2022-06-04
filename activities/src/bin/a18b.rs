// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum EmployeePosition{
    Maintenance,
    Marketing, 
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTech,
}

enum Status{
    Active,
    Terminated,
}

struct Employee{
    position:EmployeePosition,
    status:Status,
    name:String,
    age: i32,
}

// With () we just want to know if it is successfull or failed
fn try_access(employee: &Employee) -> Result<(), String>{
    match employee.status{
        Status::Terminated => return Err("This individual is no longer employed here, access has been dropped".to_owned()),
        _ => (),
    }

    match employee.position{
        EmployeePosition::Maintenance => Ok(()),
        EmployeePosition::Marketing => Ok(()),
        EmployeePosition::Manager => Ok(()),
        _ => Err("Invalid Credentials".to_owned()),
    }
}

fn print_access(employee: &Employee) -> Result<(), String>{
    let access = try_access(employee)?;
    println!("Access ok");
    Ok(())
}

fn main() {
    let manager = Employee{
        position:EmployeePosition::Manager,
        status:Status::Active,
        name:"John".to_owned(),
        age:50,
    };

    match print_access(&manager){
        Err(e) => println!("Access Denied {:?}", e),
        _ => (),
    }
}
