///Gets the text enclosed between two delimiters
/// 
///Takes a string and the position of the open delimiter
///and gets the data between the open and closed
/// 
///Panics if it cant find the paired close delimiter
/// 
/// # Examples
/// 
/// get_enclosed_text()
/// 
//FIXME - DOESNT WORK WITH NESTED DELIMITERS
#[allow(dead_code)]
pub fn get_enclosed_text(input: &String, open_position: usize, close_type: &str) -> String{
    let input_vec: Vec<char> = input[open_position..].chars().collect(); 
    const PAIRED_DELIMITERS: [char; 8] = ['{','}','[',']','(',')','<','>'];

    //Sets the delimiter to:
    //      close type if not empty,
    //      its pair if in paired,
    //      it self
    let mut delimiter = input_vec[open_position];
    if close_type != ""{
        let temp_vec: Vec<char> = close_type.chars().collect();
        delimiter = temp_vec[0];
    }
    else if let Some(paired_position) = PAIRED_DELIMITERS.iter().position(|&c| c == input_vec[open_position]){
        delimiter = PAIRED_DELIMITERS[2 * (PAIRED_DELIMITERS.len() / 2 - 1) + (paired_position % 2)];
    }

    if let Some(delimiter_position) = input_vec.iter().position(|&c| c == delimiter){
        input[open_position..delimiter_position].to_string()
    }
    else {
        panic!("No delimiter of type {} found after position {} in {}", delimiter, open_position, input);
    }
}