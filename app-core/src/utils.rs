pub fn abbreviate_number(num: i64) -> (f64, char) {
    let num= num as f64;

    if num >= 1e12 {
        return (num / 1e12, 't');
    }

    if num >= 1e9 {
        return (num / 1e9, 'b');
    }
    
    if num >= 1e6 {
        return (num / 1e6, 'm');
    }
    
    if num >= 1e3 {
        return (num / 1e3, 'k');
    }
    
    (num, '\0')
}