// rust\ssxl_cli\src\scan\report_section_formatter.rs (New File)

use nu_ansi_term::Color;
use std::fmt::Write;
// Assume utility types/structs are imported or passed in

/// Generates a formatted table of all files and their LOC.
pub fn format_file_loc_table(report_data: &ReportData) -> String {
    let mut table_output = String::new();
    // Use a fixed width or dynamic calculation to align columns
    let max_loc_width = report_data.files.iter()
        .map(|f| f.loc.to_string().len()).max().unwrap_or(0);
    
    // Header
    writeln!(table_output, "{} | {}", 
        Color::Cyan.bold().paint(format!("{:>max_loc_width$}", "LOC")),
        Color::Cyan.bold().paint("File Path")
    ).unwrap();

    // Separator
    let separator = "-".repeat(max_loc_width) + " + " + &"-".repeat(60);
    writeln!(table_output, "{}", Color::Fixed(8).paint(separator)).unwrap();

    // Rows
    for file_entry in &report_data.files {
        let loc_colored = match file_entry.loc {
            0 => Color::Black.bold().on(Color::Fixed(8)).paint(format!("{:>max_loc_width$}", file_entry.loc)),
            loc if loc > 180 => Color::Red.bold().paint(format!("{:>max_loc_width$}", loc)),
            loc if loc > 150 => Color::Yellow.bold().paint(format!("{:>max_loc_width$}", loc)),
            _ => Color::Green.paint(format!("{:>max_loc_width$}", file_entry.loc)),
        };
        writeln!(table_output, "{} | {}", loc_colored, file_entry.path).unwrap();
    }

    table_output
}

// Add other extraction functions here, like:
// pub fn format_summary_header(...) -> String { ... }