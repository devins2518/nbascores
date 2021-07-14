use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::{
        Axis, BarChart, Block, Borders, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_first_tab(f, app, chunks[1]),
        1 => draw_second_tab(f, app, chunks[1]),
        _ => unreachable!(),
    };
}

pub fn draw_empty_games<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Percentage(45),
                Constraint::Percentage(5),
                Constraint::Percentage(45),
            ]
            .as_ref(),
        )
        .split(f.size());
    let text = Span::raw("There are no games today.");
    let para = Paragraph::new(text).alignment(Alignment::Center);

    f.render_widget(para, chunks[1]);
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(9),
                Constraint::Min(8),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(area);
    draw_gauges(f, app, chunks[0]);
    draw_charts(f, app, chunks[1]);
    draw_text(f, chunks[2]);
}

fn draw_gauges<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Length(3),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(area);
    let block = Block::default().borders(Borders::ALL).title("Graphs");
    f.render_widget(block, area);

    let label = format!("{:.2}%", app._progress * 100.0);
    let gauge = Gauge::default()
        .block(Block::default().title("Gauge:"))
        .gauge_style(
            Style::default()
                .fg(Color::Magenta)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
        )
        .label(label)
        .ratio(app._progress);
    f.render_widget(gauge, chunks[0]);

    let sparkline = Sparkline::default()
        .block(Block::default().title("Sparkline:"))
        .style(Style::default().fg(Color::Green))
        .data(&app._sparkline.points)
        .bar_set(if app.enhanced_graphics {
            symbols::bar::NINE_LEVELS
        } else {
            symbols::bar::THREE_LEVELS
        });
    f.render_widget(sparkline, chunks[1]);

    let line_gauge = LineGauge::default()
        .block(Block::default().title("LineGauge:"))
        .gauge_style(Style::default().fg(Color::Magenta))
        .line_set(if app.enhanced_graphics {
            symbols::line::THICK
        } else {
            symbols::line::NORMAL
        })
        .ratio(app._progress);
    f.render_widget(line_gauge, chunks[2]);
}

fn draw_charts<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let constraints = if app._show_chart {
        vec![Constraint::Percentage(50), Constraint::Percentage(50)]
    } else {
        vec![Constraint::Percentage(100)]
    };
    let chunks = Layout::default()
        .constraints(constraints)
        .direction(Direction::Horizontal)
        .split(area);
    {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[0]);
        {
            let chunks = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .direction(Direction::Horizontal)
                .split(chunks[0]);

            // Draw tasks
            let tasks: Vec<ListItem> = app
                ._tasks
                .items
                .iter()
                .map(|i| ListItem::new(vec![Spans::from(Span::raw(*i))]))
                .collect();
            let tasks = List::new(tasks)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");
            f.render_stateful_widget(tasks, chunks[0], &mut app._tasks.state);

            // Draw logs
            let info_style = Style::default().fg(Color::Blue);
            let warning_style = Style::default().fg(Color::Yellow);
            let error_style = Style::default().fg(Color::Magenta);
            let critical_style = Style::default().fg(Color::Red);
            let logs: Vec<ListItem> = app
                ._logs
                .items
                .iter()
                .map(|&(evt, level)| {
                    let s = match level {
                        "ERROR" => error_style,
                        "CRITICAL" => critical_style,
                        "WARNING" => warning_style,
                        _ => info_style,
                    };
                    let content = vec![Spans::from(vec![
                        Span::styled(format!("{:<9}", level), s),
                        Span::raw(evt),
                    ])];
                    ListItem::new(content)
                })
                .collect();
            let logs = List::new(logs).block(Block::default().borders(Borders::ALL).title("List"));
            f.render_stateful_widget(logs, chunks[1], &mut app._logs.state);
        }

        let barchart = BarChart::default()
            .block(Block::default().borders(Borders::ALL).title("Bar chart"))
            .data(&app._barchart)
            .bar_width(3)
            .bar_gap(2)
            .bar_set(if app.enhanced_graphics {
                symbols::bar::NINE_LEVELS
            } else {
                symbols::bar::THREE_LEVELS
            })
            .value_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::ITALIC),
            )
            .label_style(Style::default().fg(Color::Yellow))
            .bar_style(Style::default().fg(Color::Green));
        f.render_widget(barchart, chunks[1]);
    }
    if app._show_chart {
        let x_labels = vec![
            Span::styled(
                format!("{}", app._signals.window[0]),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(
                "{}",
                (app._signals.window[0] + app._signals.window[1]) / 2.0
            )),
            Span::styled(
                format!("{}", app._signals.window[1]),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ];
        let datasets = vec![
            Dataset::default()
                .name("data2")
                .marker(symbols::Marker::Dot)
                .style(Style::default().fg(Color::Cyan))
                .data(&app._signals.sin1.points),
            Dataset::default()
                .name("data3")
                .marker(if app.enhanced_graphics {
                    symbols::Marker::Braille
                } else {
                    symbols::Marker::Dot
                })
                .style(Style::default().fg(Color::Yellow))
                .data(&app._signals.sin2.points),
        ];
        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .title(Span::styled(
                        "Chart",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ))
                    .borders(Borders::ALL),
            )
            .x_axis(
                Axis::default()
                    .title("X Axis")
                    .style(Style::default().fg(Color::Gray))
                    .bounds(app._signals.window)
                    .labels(x_labels),
            )
            .y_axis(
                Axis::default()
                    .title("Y Axis")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([-20.0, 20.0])
                    .labels(vec![
                        Span::styled("-20", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw("0"),
                        Span::styled("20", Style::default().add_modifier(Modifier::BOLD)),
                    ]),
            );
        f.render_widget(chart, chunks[1]);
    }
}

fn draw_text<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = vec![
        Spans::from("This is a paragraph with several lines. You can change style your text the way you want"),
        Spans::from(""),
        Spans::from(vec![
            Span::from("For example: "),
            Span::styled("under", Style::default().fg(Color::Red)),
            Span::raw(" "),
            Span::styled("the", Style::default().fg(Color::Green)),
            Span::raw(" "),
            Span::styled("rainbow", Style::default().fg(Color::Blue)),
            Span::raw("."),
        ]),
        Spans::from(vec![
            Span::raw("Oh and if you didn't "),
            Span::styled("notice", Style::default().add_modifier(Modifier::ITALIC)),
            Span::raw(" you can "),
            Span::styled("automatically", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled("wrap", Style::default().add_modifier(Modifier::REVERSED)),
            Span::raw(" your "),
            Span::styled("text", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::raw(".")
        ]),
        Spans::from(
            "One more thing is that it should display unicode characters: 10€"
        ),
    ];
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Footer",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_second_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(area);
    let titles = vec![
        Spans::from(Span::styled(
            app.boxscore.h_team.tri_code.unwrap(),
            Style::default().fg(Color::Green),
        )),
        Spans::from(Span::styled(
            app.boxscore.v_team.tri_code.unwrap(),
            Style::default().fg(Color::Green),
        )),
    ];
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Team"))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.team as usize);
    f.render_widget(tabs, chunks[0]);
    let up_style = Style::default().fg(Color::Green);
    let down_style = Style::default().fg(Color::Red);
    let rows = app.boxscore.players.iter().filter_map(|s| {
        if s.team_id.unwrap() == app.get_current_team() {
            let style = if let Some(true) = s.is_on_court {
                up_style
            } else {
                down_style
            };
            Some(
                Row::new([
                    format!("{} {}", s.first_name, s.last_name),
                    // TODO: reomve unwraps as its not always valid
                    s.pos.unwrap().to_string(),
                    format!("{: >5}", s.min.unwrap_or("0")),
                    format!("{: >5}", s.points.unwrap_or("0")),
                    format!("{: >5}", s.tot_reb.unwrap_or("0")),
                    format!("{: >5}", s.assists.unwrap_or("0")),
                    format!("{: >5}", s.steals.unwrap_or("0")),
                    format!("{: >5}", s.blocks.unwrap_or("0")),
                    // Blocked Attempts
                    format!("{: >5}", s.blocks.unwrap_or("0")),
                    format!("{: >5}", s.fgm.unwrap_or("0")),
                    format!("{: >5}", s.fga.unwrap_or("0")),
                    format!("{: >5}", s.fgp.unwrap_or("0")),
                    format!("{: >5}", s.tpm.unwrap_or("0")),
                    format!("{: >5}", s.tpa.unwrap_or("0")),
                    format!("{: >5}", s.tpp.unwrap_or("0")),
                    format!("{: >5}", s.ftm.unwrap_or("0")),
                    format!("{: >5}", s.fta.unwrap_or("0")),
                    format!("{: >5}", s.ftp.unwrap_or("0")),
                    format!("{: >5}", s.off_reb.unwrap_or("0")),
                    format!("{: >5}", s.def_reb.unwrap_or("0")),
                    format!("{: >5}", s.turnovers.unwrap_or("0")),
                    format!("{: >5}", s.p_fouls.unwrap_or("0")),
                    format!("{: >5}", s.plus_minus.unwrap_or("0")),
                ])
                .style(style),
            )
        } else {
            None
        }
    });
    let table = Table::new(rows)
        .header(
            Row::new([
                "Player", "P", "Min", "Pts", "Reb", "Ast", "Stl", "Blk", "FGM", "FGA", "FG%",
                "3PM", "3PA", "3P%", "FTM", "FTA", "FT%", "OREB", "DREB", "TOV", "PF", "+/-",
            ])
            .style(Style::default().fg(Color::Yellow))
            .bottom_margin(1),
        )
        .block(Block::default().title("Boxscore").borders(Borders::ALL))
        .widths(&[
            // TODO: Variable lengths
            // Kinda broken: https://github.com/fdehau/tui-rs/issues/499
            // Constraint::Min(0),
            Constraint::Length(15),
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
        ]);
    f.render_widget(table, chunks[1]);
}
