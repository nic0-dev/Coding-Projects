use std::collections::BinaryHeap;
use std::ops::Deref;
use std::io;

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
struct Request<T> {
    priority: usize,
    order: usize,
    book_name: T,
}

impl<T> Deref for Request<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.book_name
    }
}

fn read_input() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn sort_requests<T>(requests: &mut Vec<&Request<T>>) {    // Sort by Priority and Order
    requests.sort_by(|a, b| {
        if b.priority == a.priority {
            a.order.cmp(&b.order)
        } else {
            b.priority.cmp(&a.priority)
        }
    });
}

fn process_commands(commands: Vec<String>) -> Vec<String> {
    // Priority queue to handle book requests
    let mut heap = BinaryHeap::new();
    // Vector to store processed requests
    let mut processed_requests = Vec::new();
    let mut output = Vec::new();
    let mut order = 0;

    for cmd in commands {
        // request_book <priority> <book_name>
        // - if queue is not yet full: [UP-LIB-SYS] "<book_name>" sucessfully requested!
        // - else: [UP-LIB-SYS] Queue is full! "<book_name>" request dropped
        if cmd.starts_with("request_book") {
            let requests: Vec<&str> = cmd.splitn(3, ' ').collect();
            let priority: usize = requests[1].parse().unwrap();
            let book_name = requests[2].to_string();

            // 1. Adding books to be borrowed
            if heap.len() < 20 {    // 
                heap.push(Request { priority, order, book_name: book_name.clone() });
                order += 1;
                output.push(format!("[UP-LIB-SYS] \"{}\" successfully requested!", book_name));
            } else {
                output.push(format!("[UP-LIB-SYS] Queue is full! \"{}\" request dropped.", book_name));
            }
        } 
        // process_requests
        // - if queue is not empty: [UP-LIB-SYS] Processing <number_of_requests_in_queue> requests... Success!
        // - else: [UP-LIB-SYS] No requests to be processed!
        else if cmd == "process_requests" {
            // 2. Processing the books to be borrowed
            if heap.is_empty() {
                output.push("[UP-LIB-SYS] No requests to be processed!".to_string());
            } else {
                output.push(format!("[UP-LIB-SYS] Processing {} requests... Success!", heap.len()));
                while let Some(request) = heap.pop() {
                    processed_requests.push(request);
                }
            }
        } 
        // print_requests_in_queue
        // - if queue is not empty: [UP-LIB-SYS] Requests in queue
        //   1. <book_name> ...
        // - else: [UP-LIB-SYS] Queue is empty!
        else if cmd == "print_requests_in_queue" {
            // 3. Printing the current queue
            if heap.is_empty() {
                output.push("[UP-LIB-SYS] Queue is empty!".to_string());
            } else {
                output.push("[UP-LIB-SYS] Requests in queue".to_string());
                // Collect items and sort them by priority then order
                let mut sorted_requests = heap.iter().collect();
                sort_requests(&mut sorted_requests);
                for (i, request) in sorted_requests.iter().enumerate() {
                    output.push(format!("{}. {}", i + 1, request.book_name));
                }
            }
        } 
        // print_processed_requests
        // - [UP-LIB-SYS] Recently processed requests
        //   1. <book_name> ...
        else if cmd == "print_processed_requests" {
            // 4. Printing the books released
            output.push("[UP-LIB-SYS] Recently processed requests".to_string());
            let mut sorted_processed = processed_requests.iter().collect();
            sort_requests(&mut sorted_processed);
            for (i, request) in sorted_processed.iter().enumerate() {
                output.push(format!("{}. {}", i + 1, request.book_name));
            }
        }
    }

    output
}


fn main(){
    // run in w04_assessment folder:
    // cat in_pub.txt | ./target/release/w04_assessment.exe > out_pub.txt
    // note: to update .exe, use cargo build --release
    // Compare-Object (gc out_pub_ans.txt) (gc out_pub.txt)     

    // Enter initial requests:
    let n: usize = read_input();       // Number of initial requests
    let mut commands = Vec::new();

    // Process initial requests
    for _ in 0..n {      // N lines (Details of the request, <priority> <book_name>)
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        commands.push(format!("request_book {}", input.trim()));
    }

    // Enter number of commands:
    let t: usize = read_input();       // Number of commands

    // Process Commands
    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        commands.push(input.trim().to_string());
    }

    let output = process_commands(commands);
    for line in output {    
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_book() {
        let commands = vec!["request_book 1 BookA".to_string()];
        let expected_output = vec!["[UP-LIB-SYS] \"BookA\" successfully requested!".to_string()];
        let actual_output = process_commands(commands);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_queue_full() {  // 21 Requests
        let mut commands = vec![];
        for i in 1..=21 {   
            commands.push(format!("request_book {} Book{}", i, i));
        }
    
        let mut expected_output = vec![];
        for i in 1..=20 {
            expected_output.push(format!("[UP-LIB-SYS] \"Book{}\" successfully requested!", i));
        }
        expected_output.push("[UP-LIB-SYS] Queue is full! \"Book21\" request dropped.".to_string());
    
        let actual_output = process_commands(commands);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_process_requests() {
        let commands = vec![
            "request_book 1 BookA".to_string(),
            "request_book 2 BookB".to_string(),
            "process_requests".to_string(),
        ];
        let expected_output = vec![
            "[UP-LIB-SYS] \"BookA\" successfully requested!".to_string(),
            "[UP-LIB-SYS] \"BookB\" successfully requested!".to_string(),
            "[UP-LIB-SYS] Processing 2 requests... Success!".to_string(),
        ];
        let actual_output = process_commands(commands);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_print_requests_in_queue() {
        let commands = vec![
            "request_book 1 BookA".to_string(),
            "request_book 2 BookB".to_string(),
            "request_book 3 BookC".to_string(),
            "print_requests_in_queue".to_string(),
        ];
        let expected_output = vec![
            "[UP-LIB-SYS] \"BookA\" successfully requested!".to_string(),
            "[UP-LIB-SYS] \"BookB\" successfully requested!".to_string(),
            "[UP-LIB-SYS] \"BookC\" successfully requested!".to_string(),
            "[UP-LIB-SYS] Requests in queue".to_string(),
            "1. BookC".to_string(),
            "2. BookB".to_string(),
            "3. BookA".to_string(),
        ];
        let actual_output = process_commands(commands);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_print_processed_requests() {
        let commands = vec![
            "request_book 3 BookA".to_string(),
            "request_book 2 BookB".to_string(),
            "request_book 4 BookC".to_string(),
            "request_book 1 BookD".to_string(),
            "process_requests".to_string(),
            "print_processed_requests".to_string(),
        ];
        let expected_output = vec![
            "[UP-LIB-SYS] \"BookA\" successfully requested!".to_string(),
            "[UP-LIB-SYS] \"BookB\" successfully requested!".to_string(),
            "[UP-LIB-SYS] \"BookC\" successfully requested!".to_string(),
            "[UP-LIB-SYS] \"BookD\" successfully requested!".to_string(),
            "[UP-LIB-SYS] Processing 4 requests... Success!".to_string(),
            "[UP-LIB-SYS] Recently processed requests".to_string(),
            "1. BookC".to_string(),
            "2. BookA".to_string(),
            "3. BookB".to_string(),
            "4. BookD".to_string(),
        ];
        let actual_output = process_commands(commands);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_print_processed_requests_equal_prio() { // Test equal priority
        let commands = vec![
            "request_book 2 BookA".to_string(),
            "request_book 3 BookB".to_string(),
            "request_book 4 BookC".to_string(),
            "request_book 1 BookD".to_string(),
            "request_book 3 BookE".to_string(),
            "process_requests".to_string(),
            "print_processed_requests".to_string(),
        ];
        let expected_output = vec![
            "[UP-LIB-SYS] \"BookA\" successfully requested!".to_string(),
            "[UP-LIB-SYS] \"BookB\" successfully requested!".to_string(),
            "[UP-LIB-SYS] \"BookC\" successfully requested!".to_string(),
            "[UP-LIB-SYS] \"BookD\" successfully requested!".to_string(),
            "[UP-LIB-SYS] \"BookE\" successfully requested!".to_string(),
            "[UP-LIB-SYS] Processing 5 requests... Success!".to_string(),
            "[UP-LIB-SYS] Recently processed requests".to_string(),
            "1. BookC".to_string(),
            "2. BookB".to_string(),
            "3. BookE".to_string(),
            "4. BookA".to_string(),
            "5. BookD".to_string(),
        ];
        let actual_output = process_commands(commands);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_processing_and_printing() {
        let commands = vec![
            "request_book 1 BookA".to_string(),
            "request_book 2 BookB".to_string(),
            "print_requests_in_queue".to_string(),
            "process_requests".to_string(),
            "print_requests_in_queue".to_string(),
            "print_processed_requests".to_string(),
        ];
        let expected_output = vec![
            "[UP-LIB-SYS] \"BookA\" successfully requested!".to_string(),
            "[UP-LIB-SYS] \"BookB\" successfully requested!".to_string(),
            "[UP-LIB-SYS] Requests in queue".to_string(),
            "1. BookB".to_string(),
            "2. BookA".to_string(),
            "[UP-LIB-SYS] Processing 2 requests... Success!".to_string(),
            "[UP-LIB-SYS] Queue is empty!".to_string(),
            "[UP-LIB-SYS] Recently processed requests".to_string(),
            "1. BookB".to_string(),
            "2. BookA".to_string(),
        ];
        let actual_output = process_commands(commands);
        assert_eq!(actual_output, expected_output);
    }
}