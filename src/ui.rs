use crate::app::App;
use ratatui::layout::*;
use ratatui::prelude::*;
use ratatui::widgets::*;
use ratatui::*;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    operation_list(app, frame);
}

fn operation_list(app: &mut App, frame: &mut Frame) {
    let rows = vec![
        Row::new(vec![
            "2024/01/01",
            "This is a note for operation 1",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/02",
            "This is a note for operation 2",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/03",
            "This is a note for operation 3",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/04",
            "This is a note for operation 4",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/05",
            "This is a note for operation 5",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/06",
            "This is a note for operation 6",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/07",
            "This is a note for operation 7",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/08",
            "This is a note for operation 8",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/09",
            "This is a note for operation 9",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/10",
            "This is a note for operation 10",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/11",
            "This is a note for operation 11",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/12",
            "This is a note for operation 12",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/13",
            "This is a note for operation 13",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/14",
            "This is a note for operation 14",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/15",
            "This is a note for operation 15",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/16",
            "This is a note for operation 16",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/17",
            "This is a note for operation 17",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/18",
            "This is a note for operation 18",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/19",
            "This is a note for operation 19",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/20",
            "This is a note for operation 20",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/21",
            "This is a note for operation 21",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/22",
            "This is a note for operation 22",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/23",
            "This is a note for operation 23",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/24",
            "This is a note for operation 24",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
        Row::new(vec![
            "2024/01/25",
            "This is a note for operation 25",
            "10.00",
            "Need",
            "BNP Paribas",
            "",
        ]),
    ];
    let header = Row::new(vec!["Date", "Note", "Amount", "Purpose", "Account", "Goal"]);
    let widths = vec![
        Constraint::Min(10),
        Constraint::Percentage(100),
        Constraint::Min(8),
        Constraint::Min(11),
        Constraint::Min(20),
        Constraint::Min(20),
    ];
    let block = Block::default().borders(Borders::ALL);
    let table = Table::default()
        .rows(rows)
        .widths(widths)
        .header(header)
        .block(block)
        .highlight_style(Style::new().black().on_white())
        .column_spacing(2);
    let mut table_state = TableState::default();
    table_state.select(Some(2));
    frame.render_stateful_widget(table, frame.size(), &mut table_state);
}
