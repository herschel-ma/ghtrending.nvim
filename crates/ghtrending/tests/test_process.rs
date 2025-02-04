use ghtrending::process_devloper;

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_dev() {
        match process_devloper() {
            Ok(developers) => {
                println!("developers : {:?}", developers);
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
