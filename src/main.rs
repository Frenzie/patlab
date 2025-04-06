use slint::SharedString;

slint::slint! {
    import { TextEdit } from "std-widgets.slint";
    
    export component MainWindow inherits Window {
        width: 400px;
        height: 400px;  // Increased height for better multiline display
        title: "PatLab Regex Tester";

        callback evaluate_regex();
        
        in-out property <string> regex_input: "";
        in-out property <string> test_string: "";
        in-out property <string> result_output: "Enter a regex pattern";

        VerticalLayout {
            spacing: 8px;
            padding: 8px;
            
            Text {
                text: "Regex Pattern:";
            }
            TextEdit {
                text <=> regex_input;
                placeholder-text: "Enter regex pattern";
                edited => { evaluate_regex(); }
                height: 100px;  // Fixed height for pattern input
            }
            Text {
                text: "Test String:";
            }
            TextEdit {
                text <=> test_string;
                placeholder-text: "Enter test string";
                edited => { evaluate_regex(); }
                height: 100px;  // Taller area for multiline test string
            }
            Rectangle {
                height: 1px;
                background: #ddd;
            }
            Text {
                text: "Result:";
                font-weight: 700;
            }
            Text {
                text: result_output;
                wrap: word-wrap;  // Enable text wrapping for results
            }
        }
    }
}

fn main() {
    let main_window = MainWindow::new().unwrap();

    let weak = main_window.as_weak();
    main_window.on_evaluate_regex(move || {
        let strong = weak.upgrade().unwrap();
        let regex_input = strong.get_regex_input();
        let test_string = strong.get_test_string();

        // Don't try to evaluate empty regex
        if regex_input.is_empty() {
            strong.set_result_output(SharedString::from("Enter a regex pattern"));
            return;
        }

        let result = match regex::Regex::new(&regex_input) {
            Ok(re) => {
                if re.is_match(&test_string) {
                    SharedString::from("Match found")
                } else {
                    SharedString::from("No match found")
                }
            }
            Err(e) => SharedString::from(format!("Invalid regex: {}", e)),
        };

        strong.set_result_output(result);
    });

    main_window.run().unwrap();
}
