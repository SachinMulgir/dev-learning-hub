#[derive(Debug)]
enum DummyValues {
    Dummy1(String),
    Dummy2(String),
    Dummy3(String)
}

pub fn if_let_let_else() {
    // if let and let else:

    // if-let : It is very useful in the cases where we have to take care of some specific match condition and ignore the rest(or perform default operations for the rest).
    if_let_all();

    // let-else : It is userful when we have to check for a pattern to succeed or leave the check at once using(continue, break, panic or return.)
    let_else_all();
}

fn if_let_all() {

    let val = DummyValues::Dummy1(String::from("Dummy1 value"));

    // Syntax:
    if let DummyValues::Dummy1(value) = val {
        println!("Hi, I am assosiated value of DummyValues::Dummy1 : {value:?}");
    } else {
        println!("Hi, We are very sorry but the variable does not match the if let case.");
    }


    // if let <match_case_to_check> = <value_to_check_for_match_case> {<operation to perform when specified case if matched>}
    // where
    // if let -> Syntax to check if condition with let (combined)
    // DummyValues::Dummy1(value) -> This indicates the match condition.
    // = -> seperator for LHS and RHS
    // val -> value to be checked for the match case.

    // working -> it will check if the value(val) passed for matching matches with the match case given i.e DummyValues::Dummy1(value).
    // else block is added to perform default operation for all the cases other than match case. Equivalent to '_' placeholder.

    // Play Around:
    let val2 = DummyValues::Dummy1(String::from("Dummy1 value"));
    if let DummyValues::Dummy2(value) = val2 {
        println!("Hi, I am assosiated value of DummyValues::Dummy1 : {value:?}");
    } else {
        println!("Hi, We are very sorry but the variable does not match the if let case.");
    }
    
}

fn let_else_all() {
    let val = DummyValues::Dummy2(String::from("Dummy1 value"));
    
    // Syntax:
    let DummyValues::Dummy1(value) = val else {
        panic!("|| ERROR || Got value other than DummyValues::Dummy1(value). Therefore panicccccccccc!!!!");
    };


    // let <match_case> = <value_to_check> else{ <operation_when_the_case_is_not_matched> }
    // let : Syntax to start the expression
    // DummyValues::Dummy1(value) : Condition to check, this is a must to match else the operation will be break, panic, continue and return (based on the usage).
    // else : give the opening to else block that if the mentioned case is not matched what to be done



}
